//! Sync audio levels for the default audio device in windows with voicemeeter
use eyre::Context;
use voicemeeter::VoicemeeterRemote;
use windows::{
    Win32::{
        Media::Audio::{Endpoints::*, *},
        System::Com::*,
    },
    core::AgileReference,
};

fn main() -> Result<(), eyre::Report> {
    color_eyre::install()?;
    // first param will be the bus index as A1, A2, B1 etc.
    let bus: voicemeeter::Device = std::env::args().nth(1).unwrap_or("A1".to_owned()).parse()?;

    unsafe {
        let our_guid = CoCreateGuid()?;
        CoInitializeEx(None, COINIT_MULTITHREADED)?;

        // grab default device
        let device_enum: IMMDeviceEnumerator =
            CoCreateInstance(&MMDeviceEnumerator, None, CLSCTX_INPROC_SERVER)?;
        let endpoint_volume: IAudioEndpointVolume = device_enum
            .GetDefaultAudioEndpoint(eRender, eConsole)
            .context("couldn't get default audio device")?
            .Activate(CLSCTX_INPROC_SERVER, None)?;
        let ev = AgileReference::new(&endpoint_volume)?;
        let vm = VoicemeeterRemote::new().context("not able to get voicemeeter handle")?;

        // make changes in voicemeeter propagate to windows
        std::thread::spawn({
            let vm = vm.clone();
            let ev = ev.clone();
            move || voicemeeter_cb(ev, vm, our_guid, bus).unwrap()
        });

        // setup callback so that changes in windows are propagated
        let volume_cb: IAudioEndpointVolumeCallback = Callback::new(
            our_guid,
            vm.parameters()
                .bus(bus)
                .with_context(|| "couldn't retrieve bus")?,
        )?
        .into();
        let vcb = AgileReference::new(&volume_cb)?;
        endpoint_volume.RegisterControlChangeNotify(&volume_cb)?;
        // after registering, make sure that exit unregister the callback
        ctrlc::set_handler(move || {
            ev.resolve()
                .unwrap()
                .UnregisterControlChangeNotify(&vcb.resolve().unwrap())
                .unwrap();
            std::process::exit(0);
        })?;

        // spin forever
        loop {
            std::thread::yield_now();
        }
    }
}

fn voicemeeter_cb(
    ev: AgileReference<IAudioEndpointVolume>,
    vm: VoicemeeterRemote,
    our_guid: windows::core::GUID,
    bus: voicemeeter::Device,
) -> Result<(), eyre::Report> {
    let bus = &vm.parameters().bus(bus)?;
    // sync once to ensure that windows follows what is active in voicemeeter
    sync_vm(bus, &ev.resolve()?, &our_guid)?;
    loop {
        if vm.is_parameters_dirty()? {
            sync_vm(bus, &ev.resolve()?, &our_guid)?;
        }
        std::thread::sleep(std::time::Duration::from_millis(20));
    }
}

fn sync_vm(
    bus: &voicemeeter::interface::parameters::Bus<'_>,
    endpoint_volume: &IAudioEndpointVolume,
    our_guid: &windows::core::GUID,
) -> Result<(), eyre::Report> {
    // get the db between -60.0 and +12
    let db_scalar = (bus.gain().get()? + 60.0) / (12.0 + 60.0);
    let mute = bus.mute().get()?;
    unsafe {
        endpoint_volume.SetMasterVolumeLevelScalar(db_scalar, our_guid)?;
        endpoint_volume.SetMute(mute, our_guid)?;
    }
    Ok(())
}

#[windows::core::implement(IAudioEndpointVolumeCallback)]
struct Callback<'a> {
    bus: voicemeeter::interface::parameters::Bus<'a>,
    our_guid: windows::core::GUID,
}

impl<'a> Callback<'a> {
    fn new(
        our_guid: windows::core::GUID,
        bus: voicemeeter::interface::parameters::Bus<'a>,
    ) -> Result<Self, eyre::Report> {
        Ok(Self { our_guid, bus })
    }
}

#[allow(non_snake_case)]
impl IAudioEndpointVolumeCallback_Impl for Callback<'_> {
    fn OnNotify(&self, pnotify: *mut AUDIO_VOLUME_NOTIFICATION_DATA) -> windows::core::Result<()> {
        let changes = unsafe { pnotify.as_ref() }.unwrap();
        if changes.guidEventContext == self.our_guid {
            return Ok(());
        }
        self.bus.mute().set(changes.bMuted.as_bool()).unwrap();
        self.bus
            .fade_to()
            .set((12.0 + 60.0) * changes.fMasterVolume - 60.0, 50)
            .unwrap();
        Ok(())
    }
}
