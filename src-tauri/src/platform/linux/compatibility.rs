use std::{env, path::Path};

const DISABLE_GPU_WORKAROUNDS: &str = "PHOTON_DISABLE_LINUX_GPU_WORKAROUNDS";
const NVIDIA_EXPLICIT_SYNC: &str = "__NV_DISABLE_EXPLICIT_SYNC";

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum NvidiaStatus {
    Detected,
    NotDetected,
    Unknown,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct CompatibilityInputs {
    wayland: bool,
    nvidia: NvidiaStatus,
    explicit_sync_configured: bool,
    workarounds_disabled: bool,
}

pub fn apply_pre_init_compatibility() {
    let inputs = CompatibilityInputs {
        wayland: running_on_wayland(),
        nvidia: detect_nvidia(),
        explicit_sync_configured: env::var_os(NVIDIA_EXPLICIT_SYNC).is_some(),
        workarounds_disabled: env::var_os(DISABLE_GPU_WORKAROUNDS).is_some(),
    };

    if should_disable_nvidia_explicit_sync(inputs) {
        // This must happen before Tauri initializes GTK/WebKitGTK. Preserve an
        // explicitly supplied value, including an empty value.
        env::set_var(NVIDIA_EXPLICIT_SYNC, "1");
        eprintln!(
            "photon: linux compatibility: disabled NVIDIA explicit sync for WebKitGTK/Wayland"
        );
    }
}

fn running_on_wayland() -> bool {
    env::var_os("WAYLAND_DISPLAY")
        .map(|value| !value.is_empty())
        .unwrap_or(false)
        || env::var("XDG_SESSION_TYPE")
            .map(|value| value.eq_ignore_ascii_case("wayland"))
            .unwrap_or(false)
}

fn detect_nvidia() -> NvidiaStatus {
    let nvidia = Path::new("/sys/module/nvidia").try_exists();
    let nvidia_drm = Path::new("/sys/module/nvidia_drm").try_exists();

    match (nvidia, nvidia_drm) {
        (Ok(true), _) | (_, Ok(true)) => NvidiaStatus::Detected,
        (Ok(false), Ok(false)) => NvidiaStatus::NotDetected,
        _ => NvidiaStatus::Unknown,
    }
}

fn should_disable_nvidia_explicit_sync(inputs: CompatibilityInputs) -> bool {
    inputs.wayland
        && inputs.nvidia == NvidiaStatus::Detected
        && !inputs.explicit_sync_configured
        && !inputs.workarounds_disabled
}

#[cfg(test)]
mod tests {
    use super::*;

    fn inputs(
        wayland: bool,
        nvidia: NvidiaStatus,
        explicit_sync_configured: bool,
        workarounds_disabled: bool,
    ) -> CompatibilityInputs {
        CompatibilityInputs {
            wayland,
            nvidia,
            explicit_sync_configured,
            workarounds_disabled,
        }
    }

    #[test]
    fn enables_workaround_for_nvidia_on_wayland_without_overrides() {
        assert!(should_disable_nvidia_explicit_sync(inputs(
            true,
            NvidiaStatus::Detected,
            false,
            false,
        )));
    }

    #[test]
    fn preserves_explicit_nvidia_setting() {
        assert!(!should_disable_nvidia_explicit_sync(inputs(
            true,
            NvidiaStatus::Detected,
            true,
            false,
        )));
    }

    #[test]
    fn skips_non_nvidia_wayland_systems() {
        assert!(!should_disable_nvidia_explicit_sync(inputs(
            true,
            NvidiaStatus::NotDetected,
            false,
            false,
        )));
    }

    #[test]
    fn skips_nvidia_on_x11() {
        assert!(!should_disable_nvidia_explicit_sync(inputs(
            false,
            NvidiaStatus::Detected,
            false,
            false,
        )));
    }

    #[test]
    fn escape_hatch_skips_automatic_workaround() {
        assert!(!should_disable_nvidia_explicit_sync(inputs(
            true,
            NvidiaStatus::Detected,
            false,
            true,
        )));
    }

    #[test]
    fn unknown_gpu_is_safe() {
        assert!(!should_disable_nvidia_explicit_sync(inputs(
            true,
            NvidiaStatus::Unknown,
            false,
            false,
        )));
    }
}
