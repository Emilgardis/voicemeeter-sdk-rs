(function() {var implementors = {};
implementors["voicemeeter"] = [{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/std/error/trait.Error.html\" title=\"trait std::error::Error\">Error</a> for <a class=\"enum\" href=\"voicemeeter/interface/callback/register/enum.AudioCallbackRegisterError.html\" title=\"enum voicemeeter::interface::callback::register::AudioCallbackRegisterError\">AudioCallbackRegisterError</a>","synthetic":false,"types":["voicemeeter::interface::callback::register::AudioCallbackRegisterError"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/std/error/trait.Error.html\" title=\"trait std::error::Error\">Error</a> for <a class=\"enum\" href=\"voicemeeter/interface/callback/register/enum.AudioCallbackUnregisterError.html\" title=\"enum voicemeeter::interface::callback::register::AudioCallbackUnregisterError\">AudioCallbackUnregisterError</a>","synthetic":false,"types":["voicemeeter::interface::callback::register::AudioCallbackUnregisterError"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/std/error/trait.Error.html\" title=\"trait std::error::Error\">Error</a> for <a class=\"enum\" href=\"voicemeeter/interface/callback/start_stop/enum.AudioCallbackStartError.html\" title=\"enum voicemeeter::interface::callback::start_stop::AudioCallbackStartError\">AudioCallbackStartError</a>","synthetic":false,"types":["voicemeeter::interface::callback::start_stop::AudioCallbackStartError"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/std/error/trait.Error.html\" title=\"trait std::error::Error\">Error</a> for <a class=\"enum\" href=\"voicemeeter/interface/callback/start_stop/enum.AudioCallbackStopError.html\" title=\"enum voicemeeter::interface::callback::start_stop::AudioCallbackStopError\">AudioCallbackStopError</a>","synthetic":false,"types":["voicemeeter::interface::callback::start_stop::AudioCallbackStopError"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/std/error/trait.Error.html\" title=\"trait std::error::Error\">Error</a> for <a class=\"enum\" href=\"voicemeeter/interface/communication_login_logout/enum.LoginError.html\" title=\"enum voicemeeter::interface::communication_login_logout::LoginError\">LoginError</a>","synthetic":false,"types":["voicemeeter::interface::communication_login_logout::LoginError"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/std/error/trait.Error.html\" title=\"trait std::error::Error\">Error</a> for <a class=\"enum\" href=\"voicemeeter/interface/communication_login_logout/enum.LogoutError.html\" title=\"enum voicemeeter::interface::communication_login_logout::LogoutError\">LogoutError</a>","synthetic":false,"types":["voicemeeter::interface::communication_login_logout::LogoutError"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/std/error/trait.Error.html\" title=\"trait std::error::Error\">Error</a> for <a class=\"enum\" href=\"voicemeeter/interface/communication_login_logout/enum.RunVoicemeeterError.html\" title=\"enum voicemeeter::interface::communication_login_logout::RunVoicemeeterError\">RunVoicemeeterError</a>","synthetic":false,"types":["voicemeeter::interface::communication_login_logout::RunVoicemeeterError"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/std/error/trait.Error.html\" title=\"trait std::error::Error\">Error</a> for <a class=\"struct\" href=\"voicemeeter/interface/device/struct.GetDeviceError.html\" title=\"struct voicemeeter::interface::device::GetDeviceError\">GetDeviceError</a>","synthetic":false,"types":["voicemeeter::interface::device::GetDeviceError"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/std/error/trait.Error.html\" title=\"trait std::error::Error\">Error</a> for <a class=\"struct\" href=\"voicemeeter/interface/device/struct.GetTotalDeviceError.html\" title=\"struct voicemeeter::interface::device::GetTotalDeviceError\">GetTotalDeviceError</a>","synthetic":false,"types":["voicemeeter::interface::device::GetTotalDeviceError"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/std/error/trait.Error.html\" title=\"trait std::error::Error\">Error</a> for <a class=\"enum\" href=\"voicemeeter/interface/general_information/enum.GetVoicemeeterInformationError.html\" title=\"enum voicemeeter::interface::general_information::GetVoicemeeterInformationError\">GetVoicemeeterInformationError</a>","synthetic":false,"types":["voicemeeter::interface::general_information::GetVoicemeeterInformationError"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/std/error/trait.Error.html\" title=\"trait std::error::Error\">Error</a> for <a class=\"enum\" href=\"voicemeeter/interface/get_levels/enum.GetLevelError.html\" title=\"enum voicemeeter::interface::get_levels::GetLevelError\">GetLevelError</a>","synthetic":false,"types":["voicemeeter::interface::get_levels::GetLevelError"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/std/error/trait.Error.html\" title=\"trait std::error::Error\">Error</a> for <a class=\"enum\" href=\"voicemeeter/interface/get_levels/enum.GetMidiMessageError.html\" title=\"enum voicemeeter::interface::get_levels::GetMidiMessageError\">GetMidiMessageError</a>","synthetic":false,"types":["voicemeeter::interface::get_levels::GetMidiMessageError"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/std/error/trait.Error.html\" title=\"trait std::error::Error\">Error</a> for <a class=\"enum\" href=\"voicemeeter/interface/get_parameters/enum.GetParameterError.html\" title=\"enum voicemeeter::interface::get_parameters::GetParameterError\">GetParameterError</a>","synthetic":false,"types":["voicemeeter::interface::get_parameters::GetParameterError"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/std/error/trait.Error.html\" title=\"trait std::error::Error\">Error</a> for <a class=\"enum\" href=\"voicemeeter/interface/get_parameters/enum.IsParametersDirtyError.html\" title=\"enum voicemeeter::interface::get_parameters::IsParametersDirtyError\">IsParametersDirtyError</a>","synthetic":false,"types":["voicemeeter::interface::get_parameters::IsParametersDirtyError"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/std/error/trait.Error.html\" title=\"trait std::error::Error\">Error</a> for <a class=\"enum\" href=\"voicemeeter/interface/macro_buttons/enum.IsMacroButtonDirtyError.html\" title=\"enum voicemeeter::interface::macro_buttons::IsMacroButtonDirtyError\">IsMacroButtonDirtyError</a>","synthetic":false,"types":["voicemeeter::interface::macro_buttons::IsMacroButtonDirtyError"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/std/error/trait.Error.html\" title=\"trait std::error::Error\">Error</a> for <a class=\"enum\" href=\"voicemeeter/interface/macro_buttons/enum.GetMacroButtonStatusError.html\" title=\"enum voicemeeter::interface::macro_buttons::GetMacroButtonStatusError\">GetMacroButtonStatusError</a>","synthetic":false,"types":["voicemeeter::interface::macro_buttons::GetMacroButtonStatusError"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/std/error/trait.Error.html\" title=\"trait std::error::Error\">Error</a> for <a class=\"enum\" href=\"voicemeeter/interface/macro_buttons/enum.SetMacroButtonStatusError.html\" title=\"enum voicemeeter::interface::macro_buttons::SetMacroButtonStatusError\">SetMacroButtonStatusError</a>","synthetic":false,"types":["voicemeeter::interface::macro_buttons::SetMacroButtonStatusError"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/std/error/trait.Error.html\" title=\"trait std::error::Error\">Error</a> for <a class=\"enum\" href=\"voicemeeter/interface/set_parameters/enum.SetParametersError.html\" title=\"enum voicemeeter::interface::set_parameters::SetParametersError\">SetParametersError</a>","synthetic":false,"types":["voicemeeter::interface::set_parameters::SetParametersError"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/std/error/trait.Error.html\" title=\"trait std::error::Error\">Error</a> for <a class=\"enum\" href=\"voicemeeter/interface/set_parameters/enum.SetParameterError.html\" title=\"enum voicemeeter::interface::set_parameters::SetParameterError\">SetParameterError</a>","synthetic":false,"types":["voicemeeter::interface::set_parameters::SetParameterError"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/std/error/trait.Error.html\" title=\"trait std::error::Error\">Error</a> for <a class=\"enum\" href=\"voicemeeter/interface/set_parameters/enum.IsParametersDirtyError.html\" title=\"enum voicemeeter::interface::set_parameters::IsParametersDirtyError\">IsParametersDirtyError</a>","synthetic":false,"types":["voicemeeter::interface::set_parameters::IsParametersDirtyError"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/std/error/trait.Error.html\" title=\"trait std::error::Error\">Error</a> for <a class=\"enum\" href=\"voicemeeter/interface/enum.InitializationError.html\" title=\"enum voicemeeter::interface::InitializationError\">InitializationError</a>","synthetic":false,"types":["voicemeeter::interface::InitializationError"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/std/error/trait.Error.html\" title=\"trait std::error::Error\">Error</a> for <a class=\"enum\" href=\"voicemeeter/enum.LoadError.html\" title=\"enum voicemeeter::LoadError\">LoadError</a>","synthetic":false,"types":["voicemeeter::LoadError"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/std/error/trait.Error.html\" title=\"trait std::error::Error\">Error</a> for <a class=\"enum\" href=\"voicemeeter/enum.RemoteFileError.html\" title=\"enum voicemeeter::RemoteFileError\">RemoteFileError</a>","synthetic":false,"types":["voicemeeter::RemoteFileError"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/std/error/trait.Error.html\" title=\"trait std::error::Error\">Error</a> for <a class=\"enum\" href=\"voicemeeter/enum.RegistryError.html\" title=\"enum voicemeeter::RegistryError\">RegistryError</a>","synthetic":false,"types":["voicemeeter::RegistryError"]}];
if (window.register_implementors) {window.register_implementors(implementors);} else {window.pending_implementors = implementors;}})()