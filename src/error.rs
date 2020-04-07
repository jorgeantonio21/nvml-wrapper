use crate::ffi::bindings::*;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum Bits {
    U32(u32),
    U64(u64)
}

error_chain! {
    foreign_links {
        Utf8Error(::std::str::Utf8Error);
        NulError(::std::ffi::NulError);
    }

    errors {
        /**
        An error used to pinpoint error cause within a function to
        `PciInfo.try_into_c()`.

        This error is specific to this Rust wrapper.
        */
        PciInfoToCFailed {
            description("An error used to pinpoint error cause within a function to \
                         a call to `PciInfo.try_into_c()`.")
        }
        
        /**
        An error used to pinpoint error cause within a function to a call to
        `Device.pci_info()`.

        This error is specific to this Rust wrapper.
        */
        GetPciInfoFailed {
            description("An error used to pinpoint error cause within a function to \
                         a call to `Device.pci_info()`.")
        }

        /**
        An error used to pinpoint error cause within a function to a call to
        `EventSet.release_events()`.

        This error is specific to this Rust wrapper.
        */
        SetReleaseFailed {
            description("An error used to pinpoint error cause within a function to \
                         a call to `EventSet.release_events()`.")
        }

        /**
        A String was too long to fit into an array.

        This error is specific to this Rust wrapper.
        */
        StringTooLong(max_len: usize, actual_len: usize) {
            description("A String was too long to fit into an array.")
            display("The max String length was '{}', but the actual String \
                     length was '{}'.", max_len, actual_len)
        }

        /**
        Bits that did not correspond to a flag were encountered whilst attempting to
        interpret them as bitflags.
        
        This error is specific to this Rust wrapper.
        */
        IncorrectBits(bits: Bits) {
            description("Bits that did not correspond to a flag were encountered whilst attempting \
                        to interpret them as bitflags.")
            display("Bits that did not correspond to a flag were encountered whilst attempting \
                     to interpret them as bitflags: '{:?}'.", bits)
        }

        /**
        An unexpected enum variant was encountered.
        
        This error is specific to this Rust wrapper. It is used to represent the
        possibility that an enum variant that is not defined within the Rust bindings
        can be returned from a C call.

        The `value` field contains the value that could not be mapped to a
        defined enum variant.

        See <https://github.com/rust-lang/rust/issues/36927>
        */
        UnexpectedVariant(value: i32) {
            description("An unexpected enum variant was encountered.")
            display("The unexpected value '{}' was encountered and could not be \
                     mapped to a defined enum variant.", value)
        }

        /// NVML was not first initialized with `NVML::init()`.
        Uninitialized {
            description("NVML was not first initialized with `NVML::init()`.")
        }

        /// A supplied argument is invalid.
        InvalidArg {
            description("A supplied argument is invalid.")
        }

        /// The requested operation is not available on the target device.
        NotSupported {
            description("The requested operation is not available on the target device.")
        }

        /// The current user does not have permission for the operation.
        NoPermission {
            description("The current user does not have permission for the operation.")
        }

        /// This error is deprecated on the part of the NVML lib itself and should
        /// not be encountered. Multiple initializations are now allowed through refcounting.
        AlreadyInitialized {
            description("This error is deprecated on the part of the NVML lib itself and should \
                        not be encountered. Multiple initializations are now allowed through refcounting.")
        }

        /// A query to find an object was unsuccessful.
        NotFound {
            description("A query to find and object was unsuccessful.")
        }

        /**
        An input argument is not large enough.
        
        The value contained is the size required for a successful call (if `Some`)
        and `None` if not explicitly set.
        */
        InsufficientSize(required_size: Option<usize>) {
            description("An input argument is not large enough.")
            display("An input argument is not large enough. Required size: '{:?}'", required_size)
        }

        /// A device's external power cables are not properly attached.
        InsufficientPower {
            description("A device's external power cables are not properly attached.")
        }

        /// NVIDIA driver is not loaded.
        DriverNotLoaded {
            description("NVIDIA driver is not loaded.")
        }

        /// User provided timeout passed.
        Timeout {
            description("User provided timeout passed.")
        }

        /// NVIDIA kernel detected an interrupt issue with a GPU.
        IrqIssue {
            description("NVIDIA kernel detected an interrupt issue with a GPU.")
        }

        /// A shared library couldn't be found or loaded.
        LibraryNotFound {
            description("A shared library couldn't be found or loaded.")
        }

        /// A function could not be found in a shared library.
        FunctionNotFound {
            description("A function could not be found in a shared library")
        }

        /// infoROM is corrupted.
        CorruptedInfoROM {
            description("infoROM is corrupted.")
        }

        /// The GPU has fallen off the bus or has otherwise become inaccessible.
        GpuLost {
            description("The GPU has fallen off the bus or has otherwise become inaccessible.")
        }

        /// The GPU requires a reset before it can be used again.
        ResetRequired {
            description("The GPU requires a reset before it can be used again.")
        }

        /// The GPU control device has been blocked by the operating system/cgroups.
        OperatingSystem {
            description("The GPU control device has been blocked by the operating system/cgroups.")
        }

        /// RM detects a driver/library version mismatch.
        LibRmVersionMismatch {
            description("RM detects a driver/library version mismatch.")
        }

        /// An operation cannot be performed because the GPU is currently in use.
        InUse {
            description("An operation cannot be performed because the GPU is currently in use.")
        }

        InsufficientMemory {
            description("Insufficient memory.")
        }

        /// No data.
        NoData {
            description("No data.")
        }

        /// The requested vgpu operation is not available on the target device because
        /// ECC is enabled.
        VgpuEccNotSupported {
            description("The requested vgpu operation is not available on the target \
                        device because ECC is enabled.")
        }

        /// An internal driver error occurred.
        Unknown {
            description("An internal driver error occurred.")
        }
    }
}

/// Converts an `nvmlReturn_t` type into a `Result<(), Error>`.
#[doc(hidden)]
pub fn nvml_try(code: nvmlReturn_t) -> Result<()> {
    match code {
        nvmlReturn_enum_NVML_SUCCESS => Ok(()),
        nvmlReturn_enum_NVML_ERROR_UNINITIALIZED => Err(Error::from_kind(ErrorKind::Uninitialized)),
        nvmlReturn_enum_NVML_ERROR_INVALID_ARGUMENT => Err(Error::from_kind(ErrorKind::InvalidArg)),
        nvmlReturn_enum_NVML_ERROR_NOT_SUPPORTED => Err(Error::from_kind(ErrorKind::NotSupported)),
        nvmlReturn_enum_NVML_ERROR_NO_PERMISSION => Err(Error::from_kind(ErrorKind::NoPermission)),
        nvmlReturn_enum_NVML_ERROR_ALREADY_INITIALIZED => Err(
            Error::from_kind(ErrorKind::AlreadyInitialized)
        ),
        nvmlReturn_enum_NVML_ERROR_NOT_FOUND => Err(Error::from_kind(ErrorKind::NotFound)),
        nvmlReturn_enum_NVML_ERROR_INSUFFICIENT_SIZE => Err(
            Error::from_kind(ErrorKind::InsufficientSize(None))
        ),
        nvmlReturn_enum_NVML_ERROR_INSUFFICIENT_POWER => Err(
            Error::from_kind(ErrorKind::InsufficientPower)
        ),
        nvmlReturn_enum_NVML_ERROR_DRIVER_NOT_LOADED => Err(
            Error::from_kind(ErrorKind::DriverNotLoaded)
        ),
        nvmlReturn_enum_NVML_ERROR_TIMEOUT => Err(Error::from_kind(ErrorKind::Timeout)),
        nvmlReturn_enum_NVML_ERROR_IRQ_ISSUE => Err(Error::from_kind(ErrorKind::IrqIssue)),
        nvmlReturn_enum_NVML_ERROR_LIBRARY_NOT_FOUND => Err(
            Error::from_kind(ErrorKind::LibraryNotFound)
        ),
        nvmlReturn_enum_NVML_ERROR_FUNCTION_NOT_FOUND => Err(
            Error::from_kind(ErrorKind::FunctionNotFound)
        ),
        nvmlReturn_enum_NVML_ERROR_CORRUPTED_INFOROM => Err(
            Error::from_kind(ErrorKind::CorruptedInfoROM)
        ),
        nvmlReturn_enum_NVML_ERROR_GPU_IS_LOST => Err(Error::from_kind(ErrorKind::GpuLost)),
        nvmlReturn_enum_NVML_ERROR_RESET_REQUIRED => Err(
            Error::from_kind(ErrorKind::ResetRequired)
        ),
        nvmlReturn_enum_NVML_ERROR_OPERATING_SYSTEM => Err(
            Error::from_kind(ErrorKind::OperatingSystem)
        ),
        nvmlReturn_enum_NVML_ERROR_LIB_RM_VERSION_MISMATCH => Err(
            Error::from_kind(ErrorKind::LibRmVersionMismatch)
        ),
        nvmlReturn_enum_NVML_ERROR_IN_USE => Err(Error::from_kind(ErrorKind::InUse)),
        nvmlReturn_enum_NVML_ERROR_MEMORY => Err(Error::from_kind(ErrorKind::InsufficientMemory)),
        nvmlReturn_enum_NVML_ERROR_NO_DATA => Err(Error::from_kind(ErrorKind::NoData)),
        nvmlReturn_enum_NVML_ERROR_VGPU_ECC_NOT_SUPPORTED => Err(Error::from_kind(ErrorKind::VgpuEccNotSupported)),
        nvmlReturn_enum_NVML_ERROR_UNKNOWN => Err(Error::from_kind(ErrorKind::Unknown)),
        _ => Err(Error::from_kind(ErrorKind::UnexpectedVariant(code))),
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn nvml_try_success() {
        let res = nvml_try(nvmlReturn_enum_NVML_SUCCESS);
        assert_eq!(res.unwrap(), ())
    }

    #[test]
    fn nvml_try_uninitialized() {
        let res = nvml_try(nvmlReturn_enum_NVML_ERROR_UNINITIALIZED);
        assert_matches!(res, Err(Error(ErrorKind::Uninitialized, _)));
    }

    #[test]
    fn nvml_try_invalid_argument() {
        let res = nvml_try(nvmlReturn_enum_NVML_ERROR_INVALID_ARGUMENT);
        assert_matches!(res, Err(Error(ErrorKind::InvalidArg, _)))
    }

    #[test]
    fn nvml_try_not_supported() {
        let res = nvml_try(nvmlReturn_enum_NVML_ERROR_NOT_SUPPORTED);
        assert_matches!(res, Err(Error(ErrorKind::NotSupported, _)))
    }

    #[test]
    fn nvml_try_no_permission() {
        let res = nvml_try(nvmlReturn_enum_NVML_ERROR_NO_PERMISSION);
        assert_matches!(res, Err(Error(ErrorKind::NoPermission, _)))
    }

    #[test]
    fn nvml_try_already_initialized() {
        let res = nvml_try(nvmlReturn_enum_NVML_ERROR_ALREADY_INITIALIZED);
        assert_matches!(res, Err(Error(ErrorKind::AlreadyInitialized, _)))
    }

    #[test]
    fn nvml_try_not_found() {
        let res = nvml_try(nvmlReturn_enum_NVML_ERROR_NOT_FOUND);
        assert_matches!(res, Err(Error(ErrorKind::NotFound, _)))
    }

    #[test]
    fn nvml_try_insufficient_size() {
        let res = nvml_try(nvmlReturn_enum_NVML_ERROR_INSUFFICIENT_SIZE);
        assert_matches!(res, Err(Error(ErrorKind::InsufficientSize(None), _)))
    }

    #[test]
    fn nvml_try_insufficient_power() {
        let res = nvml_try(nvmlReturn_enum_NVML_ERROR_INSUFFICIENT_POWER);
        assert_matches!(res, Err(Error(ErrorKind::InsufficientPower, _)))
    }

    #[test]
    fn nvml_try_driver_not_loaded() {
        let res = nvml_try(nvmlReturn_enum_NVML_ERROR_DRIVER_NOT_LOADED);
        assert_matches!(res, Err(Error(ErrorKind::DriverNotLoaded, _)))
    }

    #[test]
    fn nvml_try_timeout() {
        let res = nvml_try(nvmlReturn_enum_NVML_ERROR_TIMEOUT);
        assert_matches!(res, Err(Error(ErrorKind::Timeout, _)))
    }

    #[test]
    fn nvml_try_irq_issue() {
        let res = nvml_try(nvmlReturn_enum_NVML_ERROR_IRQ_ISSUE);
        assert_matches!(res, Err(Error(ErrorKind::IrqIssue, _)))
    }

    #[test]
    fn nvml_try_library_not_found() {
        let res = nvml_try(nvmlReturn_enum_NVML_ERROR_LIBRARY_NOT_FOUND);
        assert_matches!(res, Err(Error(ErrorKind::LibraryNotFound, _)))
    }

    #[test]
    fn nvml_try_function_not_found() {
        let res = nvml_try(nvmlReturn_enum_NVML_ERROR_FUNCTION_NOT_FOUND);
        assert_matches!(res, Err(Error(ErrorKind::FunctionNotFound, _)))
    }

    #[test]
    fn nvml_try_corrupted_inforom() {
        let res = nvml_try(nvmlReturn_enum_NVML_ERROR_CORRUPTED_INFOROM);
        assert_matches!(res, Err(Error(ErrorKind::CorruptedInfoROM, _)))
    }

    #[test]
    fn nvml_try_gpu_is_lost() {
        let res = nvml_try(nvmlReturn_enum_NVML_ERROR_GPU_IS_LOST);
        assert_matches!(res, Err(Error(ErrorKind::GpuLost, _)))
    }

    #[test]
    fn nvml_try_reset_required() {
        let res = nvml_try(nvmlReturn_enum_NVML_ERROR_RESET_REQUIRED);
        assert_matches!(res, Err(Error(ErrorKind::ResetRequired, _)))
    }

    #[test]
    fn nvml_try_operating_system() {
        let res = nvml_try(nvmlReturn_enum_NVML_ERROR_OPERATING_SYSTEM);
        assert_matches!(res, Err(Error(ErrorKind::OperatingSystem, _)))
    }

    #[test]
    fn nvml_try_lib_rm_version_mismatch() {
        let res = nvml_try(nvmlReturn_enum_NVML_ERROR_LIB_RM_VERSION_MISMATCH);
        assert_matches!(res, Err(Error(ErrorKind::LibRmVersionMismatch, _)))
    }

    #[test]
    fn nvml_try_in_use() {
        let res = nvml_try(nvmlReturn_enum_NVML_ERROR_IN_USE);
        assert_matches!(res, Err(Error(ErrorKind::InUse, _)))
    }

    #[test]
    fn nvml_try_memory() {
        let res = nvml_try(nvmlReturn_enum_NVML_ERROR_MEMORY);
        assert_matches!(res, Err(Error(ErrorKind::InsufficientMemory, _)))
    }

    #[test]
    fn nvml_try_no_data() {
        let res = nvml_try(nvmlReturn_enum_NVML_ERROR_NO_DATA);
        assert_matches!(res, Err(Error(ErrorKind::NoData, _)))
    }

    #[test]
    fn nvml_try_vgpu_ecc_not_supported() {
        let res = nvml_try(nvmlReturn_enum_NVML_ERROR_VGPU_ECC_NOT_SUPPORTED);
        assert_matches!(res, Err(Error(ErrorKind::VgpuEccNotSupported, _)))
    }

    #[test]
    fn nvml_try_unknown() {
        let res = nvml_try(nvmlReturn_enum_NVML_ERROR_UNKNOWN);
        assert_matches!(res, Err(Error(ErrorKind::Unknown, _)))
    }
}
