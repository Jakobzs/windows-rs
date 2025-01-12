fn main() {
    windows::build!(
        Microsoft::Web::WebView2::Core::*,
        Windows::Foundation::*,
        Windows::Win32::Graphics::Gdi::UpdateWindow,
        Windows::Win32::System::SystemServices::{GetModuleHandleA, HINSTANCE, LRESULT, PWSTR},
        Windows::Win32::System::Threading::GetCurrentThreadId,
        Windows::Win32::UI::DisplayDevices::{POINT, RECT, SIZE},
        Windows::Win32::UI::HiDpi::{
            PROCESS_DPI_AWARENESS, SetProcessDpiAwareness, PROCESS_PER_MONITOR_DPI_AWARE
        },
        Windows::Win32::UI::KeyboardAndMouseInput::SetFocus,
        Windows::Win32::UI::MenusAndResources::HMENU,
        Windows::Win32::UI::WindowsAndMessaging::*
    )
}
