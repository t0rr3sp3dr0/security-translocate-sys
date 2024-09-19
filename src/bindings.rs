use crate::annotations::{_Nonnull, _Nullable};
use core_foundation::dictionary::CFDictionaryRef;
use core_foundation::error::CFErrorRef;
use core_foundation::url::CFURLRef;
use libc::pid_t;

#[link(name = "Security", kind = "framework")]
extern "C" {
    /// Initialize the SecTranslocate Library as the XPC Server, Disk Arbitration Listener, and Launch Services Notification listener
    ///
    /// # Arguments
    /// * `error` - On error will be populated with an error object describing the failure (a posix domain error such as EINVAL)
    ///
    /// # Return
    /// true on success false on failure
    pub fn SecTranslocateStartListening(error: _Nullable<*mut CFErrorRef>) -> bool; // __OSX_AVAILABLE(10.12)

    /// Initialize the SecTranslocate Library as the XPC Server, Disk Arbitration Listener, and Launch Services Notification listener
    ///
    /// # Arguments
    /// * `options` - (currently unused) A dictionary of options that could impact server startup
    /// * `out_error` - On error will be populated with an error object describing the failure (a posix domain error such as EINVAL)
    ///
    /// # Return
    /// true on success false on failure
    pub fn SecTranslocateStartListeningWithOptions(
        options: _Nonnull<CFDictionaryRef>,
        out_error: _Nullable<*mut CFErrorRef>,
    ) -> bool; // __OSX_AVAILABLE(10.12)

    /// Create a CFURL pointing to a translocated location from which to access the directory specified by pathToTranslocate.
    ///
    /// # Arguments
    /// * `path_to_translocate` - URL of the directory to be accessed from a translocated location.
    /// * `destination_path` - URL where the directory of interest should be translocated, or NULL for a random UUID location
    /// * `error` - On error will be populated with an error object describing the failure (a posix domain error such as EINVAL)
    ///
    /// # Return
    /// A CFURL pointing to the translocated location of the directory.
    ///
    /// # Discussion
    /// <https://github.com/apple-oss-distributions/Security/blob/rel/Security-59754/OSX/libsecurity_translocate/lib/SecTranslocate.h#L71-L96>
    pub fn SecTranslocateCreateSecureDirectoryForURL(
        path_to_translocate: _Nonnull<CFURLRef>,
        destination_path: _Nullable<CFURLRef>,
        error: _Nullable<*mut CFErrorRef>,
    ) -> _Nullable<CFURLRef>; // __OSX_AVAILABLE(10.12)

    /// Create a CFURL pointing to a translocated location from which to access the directory specified by pathToTranslocate.
    ///
    /// # Arguments
    /// * `path_to_translocate` - URL of the directory to be accessed from a translocated location.
    /// * `destination_path` - URL where the directory of interest should be translocated
    /// * `error` - On error will be populated with an error object describing the failure (a posix domain error such as EINVAL)
    ///
    /// # Return
    /// A CFURL pointing to the translocated location of the directory.
    ///
    /// # Discussion
    /// <https://github.com/apple-oss-distributions/Security/blob/rel/Security-59754/OSX/libsecurity_translocate/lib/SecTranslocate.h#L112-L123>
    pub fn SecTranslocateCreateGeneric(
        path_to_translocate: _Nonnull<CFURLRef>,
        destination_path: _Nonnull<CFURLRef>,
        error: _Nullable<*mut CFErrorRef>,
    ) -> _Nullable<CFURLRef>; // __OSX_AVAILABLE(10.16)

    /// Register that a translocated pid is running
    ///
    /// # Arguments
    /// * `pid` - the pid to register
    ///
    /// # Discussion
    /// <https://github.com/apple-oss-distributions/Security/blob/rel/Security-59754/OSX/libsecurity_translocate/lib/SecTranslocate.h#L135-L137>
    pub fn SecTranslocateAppLaunchCheckin(pid: pid_t); // __OSX_AVAILABLE(10.12)

    /// Implements policy to decide whether the entity defined by path should be run translocated
    ///
    /// # Arguments
    /// * `path` - URL to the entity in question
    /// * `should_translocate` - true if the path should be translocated, false otherwise
    /// * `error` - On error will be populated with an error object describing the failure (a posix domain error such as EINVAL)
    ///
    /// # Return
    /// true on success, false on failure (on failure error is set if provided). shouldTranslocate gives the answer
    ///
    /// # Discussion
    /// <https://github.com/apple-oss-distributions/Security/blob/rel/Security-59754/OSX/libsecurity_translocate/lib/SecTranslocate.h#L155-L161>
    pub fn SecTranslocateURLShouldRunTranslocated(
        path: _Nonnull<CFURLRef>,
        should_translocate: _Nonnull<*mut bool>,
        error: _Nullable<*mut CFErrorRef>,
    ) -> bool; // __OSX_AVAILABLE(10.12)

    /// indicates whether the provided path is an original path or a translocated path
    ///
    /// # Arguments
    /// * `path` - path to check
    /// * `is_translocated` - true if the path is translocated, false otherwise
    /// * `error` - On error will be populated with an error object describing the failure (a posix domain error such as EINVAL)
    ///
    /// # Return
    /// true on success, false on failure (on failure error is set if provided). isTranslocated gives the answer
    ///
    /// # Discussion
    /// <https://github.com/apple-oss-distributions/Security/blob/rel/Security-59754/OSX/libsecurity_translocate/lib/SecTranslocate.h#L179-L188>
    pub fn SecTranslocateIsTranslocatedURL(
        path: _Nonnull<CFURLRef>,
        is_translocated: _Nonnull<*mut bool>,
        error: _Nullable<*mut CFErrorRef>,
    ) -> bool; // __OSX_AVAILABLE(10.12)

    /// finds the original path to a file given a translocated path
    ///
    /// # Arguments
    /// * `translocated_path` - the path to look up
    /// * `error` - On error will be populated with an error object describing the failure (a posix domain error such as EINVAL)
    ///
    /// # Return
    /// A valid, existant path, or NULL on error
    ///
    /// # Discussion
    /// <https://github.com/apple-oss-distributions/Security/blob/rel/Security-59754/OSX/libsecurity_translocate/lib/SecTranslocate.h#L204-L213>
    pub fn SecTranslocateCreateOriginalPathForURL(
        translocated_path: _Nonnull<CFURLRef>,
        error: _Nullable<*mut CFErrorRef>,
    ) -> _Nullable<CFURLRef>; // __OSX_AVAILABLE(10.12)

    /// Unmount the translocated directory structure and delete the mount point directory.
    ///
    /// # Arguments
    /// * `translocated_path` - a CFURL pointing to a translocated location.
    /// * `error` - On error will be populated with an error object describing the failure (a posix domain error such as EINVAL).
    ///
    /// # Return
    /// true on success, false on error.
    ///
    /// # Discussion
    /// <https://github.com/apple-oss-distributions/Security/blob/rel/Security-59754/OSX/libsecurity_translocate/lib/SecTranslocate.h#L229-L232>
    pub fn SecTranslocateDeleteSecureDirectory(
        translocated_path: _Nonnull<CFURLRef>,
        error: _Nullable<*mut CFErrorRef>,
    ) -> bool; // __OSX_AVAILABLE(10.12)
}
