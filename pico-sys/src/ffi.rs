/* automatically generated by rust-bindgen

bindgen --whitelist-type 'pico.*' --whitelist-var '(pico|PICO).*' --whitelist-function 'pico.*' picoapi.h
*/
#![allow(non_camel_case_types)]

pub const PICO_MAX_VOICE_NAME_SIZE: u32 = 32;
pub const PICO_MAX_RESOURCE_NAME_SIZE: u32 = 32;
pub const PICO_MAX_DATAPATH_NAME_SIZE: u32 = 128;
pub const PICO_MAX_FILE_NAME_SIZE: u32 = 64;
pub const PICO_MAX_NUM_RESOURCES: u32 = 64;
pub const PICO_MAX_NUM_VOICE_DEFINITIONS: u32 = 64;
pub const PICO_MAX_NUM_RSRC_PER_VOICE: u32 = 16;
pub const PICO_MAX_FOREIGN_HEADER_LEN: u32 = 64;
pub const PICO_RESET_FULL: u32 = 0;
pub const PICO_RESET_SOFT: u32 = 16;
pub const PICO_INT16_MAX: u32 = 32767;
pub const PICO_UINT16_MAX: u32 = 65535;
pub const PICO_INT32_MAX: u32 = 2147483647;
pub const PICO_UINT32_MAX: u32 = 4294967295;
pub const PICO_RETSTRINGSIZE: u32 = 200;
pub type pico_Status = ::std::os::raw::c_int;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pico_system {
    _unused: [u8; 0],
}
pub type pico_System = *mut pico_system;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pico_resource {
    _unused: [u8; 0],
}
pub type pico_Resource = *mut pico_resource;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct pico_engine {
    _unused: [u8; 0],
}
pub type pico_Engine = *mut pico_engine;
pub type pico_Int16 = ::std::os::raw::c_short;
pub type pico_Uint16 = ::std::os::raw::c_ushort;
pub type pico_Int32 = ::std::os::raw::c_int;
pub type pico_Uint32 = ::std::os::raw::c_uint;
pub type pico_Char = ::std::os::raw::c_uchar;
pub type pico_Retstring = [::std::os::raw::c_char; 200usize];
extern "C" {
    #[doc = "Initializes the Pico system and returns its handle in \'outSystem\'."]
    #[doc = "\'memory\' and \'size\' define the location and maximum size of memory"]
    #[doc = "in number of bytes that the Pico system will use. The minimum size"]
    #[doc = "required depends on the number of engines and configurations of"]
    #[doc = "lingware to be used. No additional memory will be allocated by the"]
    #[doc = "Pico system. This function must be called before any other API"]
    #[doc = "function is called. It may only be called once (e.g. at application"]
    #[doc = "startup), unless a call to \'pico_terminate\'."]
    pub fn pico_initialize(
        memory: *mut ::std::os::raw::c_void,
        size: pico_Uint32,
        outSystem: *mut pico_System,
    ) -> pico_Status;
}
extern "C" {
    #[doc = "Terminates the Pico system. Lingware resources still being loaded"]
    #[doc = "are unloaded automatically. The memory area provided to Pico in"]
    #[doc = "\'pico_initialize\' is released. The system handle becomes"]
    #[doc = "invalid. It is not allowed to call this function as long as Pico"]
    #[doc = "engine instances are existing. No API function may be called after"]
    #[doc = "this function, except for \'pico_initialize\', which reinitializes"]
    #[doc = "the system."]
    pub fn pico_terminate(system: *mut pico_System) -> pico_Status;
}
extern "C" {
    #[doc = "Returns in \'outMessage\' a description of the system status or of an"]
    #[doc = "error that occurred with the most recently called system-level API"]
    #[doc = "function."]
    pub fn pico_getSystemStatusMessage(
        system: pico_System,
        errCode: pico_Status,
        outMessage: *mut ::std::os::raw::c_char,
    ) -> pico_Status;
}
extern "C" {
    #[doc = "Returns in \'outNrOfWarnings\' the number of warnings that occurred"]
    #[doc = "with the most recently called system-level API function."]
    pub fn pico_getNrSystemWarnings(
        system: pico_System,
        outNrOfWarnings: *mut pico_Int32,
    ) -> pico_Status;
}
extern "C" {
    #[doc = "Returns in \'outMessage\' a description of a warning that occurred"]
    #[doc = "with the most recently called system-level API function."]
    #[doc = "\'warningIndex\' must be in the range 0..N-1 where N is the number of"]
    #[doc = "warnings returned by \'pico_getNrSystemWarnings\'. \'outCode\' returns"]
    #[doc = "the warning as an integer code (cf. PICO_WARN_*)."]
    pub fn pico_getSystemWarning(
        system: pico_System,
        warningIndex: pico_Int32,
        outCode: *mut pico_Status,
        outMessage: *mut ::std::os::raw::c_char,
    ) -> pico_Status;
}
extern "C" {
    #[doc = "Loads a resource file into the Pico system. The number of resource"]
    #[doc = "files loaded in parallel is limited by PICO_MAX_NUM_RESOURCES."]
    #[doc = "Loading of a resource file may be done at any time (even in"]
    #[doc = "parallel to a running engine doing TTS synthesis), but with the"]
    #[doc = "general restriction that functions taking a system handle as their"]
    #[doc = "first argument must be called in a mutually exclusive fashion. The"]
    #[doc = "loaded resource will be available only to engines started after the"]
    #[doc = "resource is fully loaded, i.e., not to engines currently"]
    #[doc = "running."]
    pub fn pico_loadResource(
        system: pico_System,
        resourceFileName: *const pico_Char,
        outResource: *mut pico_Resource,
    ) -> pico_Status;
}
extern "C" {
    #[doc = "Unloads a resource file from the Pico system. If no engine uses the"]
    #[doc = "resource file, the resource is removed immediately and its"]
    #[doc = "associated internal memory is released, otherwise"]
    #[doc = "PICO_EXC_RESOURCE_BUSY is returned."]
    pub fn pico_unloadResource(
        system: pico_System,
        inoutResource: *mut pico_Resource,
    ) -> pico_Status;
}
extern "C" {
    #[doc = "Gets the unique resource name of a loaded resource"]
    pub fn pico_getResourceName(
        system: pico_System,
        resource: pico_Resource,
        outName: *mut ::std::os::raw::c_char,
    ) -> pico_Status;
}
extern "C" {
    #[doc = "Creates a voice definition. Resources must be added to the created"]
    #[doc = "voice with \'pico_addResourceToVoiceDefinition\' before using the"]
    #[doc = "voice in \'pico_newEngine\'. It is an error to create a voice"]
    #[doc = "definition with a previously defined voice name. In that case use"]
    #[doc = "\'pico_releaseVoiceName\' first."]
    pub fn pico_createVoiceDefinition(
        system: pico_System,
        voiceName: *const pico_Char,
    ) -> pico_Status;
}
extern "C" {
    #[doc = "Adds a mapping pair (\'voiceName\', \'resourceName\') to the voice"]
    #[doc = "definition. Multiple mapping pairs can added to a voice defintion."]
    #[doc = "When calling \'pico_newEngine\' with \'voiceName\', the corresponding"]
    #[doc = "resources from the mappings will be used with that engine."]
    pub fn pico_addResourceToVoiceDefinition(
        system: pico_System,
        voiceName: *const pico_Char,
        resourceName: *const pico_Char,
    ) -> pico_Status;
}
extern "C" {
    #[doc = "Releases the voice definition \'voiceName\'."]
    pub fn pico_releaseVoiceDefinition(
        system: pico_System,
        voiceName: *const pico_Char,
    ) -> pico_Status;
}
extern "C" {
    #[doc = "Creates and initializes a new Pico engine instance and returns its"]
    #[doc = "handle in \'outEngine\'. Only one instance per system is currently"]
    #[doc = "possible."]
    pub fn pico_newEngine(
        system: pico_System,
        voiceName: *const pico_Char,
        outEngine: *mut pico_Engine,
    ) -> pico_Status;
}
extern "C" {
    #[doc = "Disposes a Pico engine and releases all memory it occupied. The"]
    #[doc = "engine handle becomes invalid."]
    pub fn pico_disposeEngine(system: pico_System, inoutEngine: *mut pico_Engine) -> pico_Status;
}
extern "C" {
    #[doc = "Puts text \'text\' encoded in UTF8 into the Pico text input buffer."]
    #[doc = "\'textSize\' is the maximum size in number of bytes accessible in"]
    #[doc = "\'text\'. The input text may also contain text-input commands to"]
    #[doc = "change, for example, speed or pitch of the resulting speech"]
    #[doc = "output. The number of bytes actually copied to the Pico text input"]
    #[doc = "buffer is returned in \'outBytesPut\'. Sentence ends are"]
    #[doc = "automatically detected. \'\\0\' characters may be embedded in \'text\'"]
    #[doc = "to finish text input or separate independently to be synthesized"]
    #[doc = "text parts from each other. Repeatedly calling \'pico_getData\' will"]
    #[doc = "result in the content of the text input buffer to be synthesized"]
    #[doc = "(up to the last sentence end or \'\\0\' character detected). To empty"]
    #[doc = "the internal buffers without finishing synthesis, use the function"]
    #[doc = "\'pico_resetEngine\'."]
    pub fn pico_putTextUtf8(
        engine: pico_Engine,
        text: *const pico_Char,
        textSize: pico_Int16,
        outBytesPut: *mut pico_Int16,
    ) -> pico_Status;
}
extern "C" {
    #[doc = "Gets speech data from the engine. Every time this function is"]
    #[doc = "called, the engine performs, within a short time slot, a small"]
    #[doc = "amount of processing its input text, and then gives control back to"]
    #[doc = "the calling application. Ie. after calling \'pico_putTextUtf8\'"]
    #[doc = "(incl. a final embedded \'\\0\'), this function needs to be called"]
    #[doc = "repeatedly till \'outBytesReceived\' bytes are returned in"]
    #[doc = "\'outBuffer\'. The type of data returned in \'outBuffer\' (e.g. 8 or 16"]
    #[doc = "bit PCM samples) is returned in \'outDataType\' and depends on the"]
    #[doc = "lingware resources. Possible \'outDataType\' values are listed in"]
    #[doc = "picodefs.h (PICO_DATA_*)."]
    #[doc = "This function returns PICO_STEP_BUSY while processing input and"]
    #[doc = "producing speech output. Once all data is returned and there is no"]
    #[doc = "more input text available in the Pico text input buffer,"]
    #[doc = "PICO_STEP_IDLE is returned.  All other function return values"]
    #[doc = "indicate a system error."]
    pub fn pico_getData(
        engine: pico_Engine,
        outBuffer: *mut ::std::os::raw::c_void,
        bufferSize: pico_Int16,
        outBytesReceived: *mut pico_Int16,
        outDataType: *mut pico_Int16,
    ) -> pico_Status;
}
extern "C" {
    #[doc = "Resets the engine and clears all engine-internal buffers, in"]
    #[doc = "particular text input and signal data output buffers."]
    #[doc = "\'resetMode\' is one of \'PICO_RESET_SOFT\', to be used to flush the engine,"]
    #[doc = "or \'PICO_RESET_FULL\', to reset the engine after an engine error."]
    pub fn pico_resetEngine(engine: pico_Engine, resetMode: pico_Int32) -> pico_Status;
}
extern "C" {
    #[doc = "Returns in \'outMessage\' a description of the engine status or of an"]
    #[doc = "error that occurred with the most recently called engine-level API"]
    #[doc = "function."]
    pub fn pico_getEngineStatusMessage(
        engine: pico_Engine,
        errCode: pico_Status,
        outMessage: *mut ::std::os::raw::c_char,
    ) -> pico_Status;
}
extern "C" {
    #[doc = "Returns in \'outNrOfWarnings\' the number of warnings that occurred"]
    #[doc = "with the most recently called engine-level API function."]
    pub fn pico_getNrEngineWarnings(
        engine: pico_Engine,
        outNrOfWarnings: *mut pico_Int32,
    ) -> pico_Status;
}
extern "C" {
    #[doc = "Returns in \'outMessage\' a description of a warning that occurred"]
    #[doc = "with the most recently called engine-level API function."]
    #[doc = "\'warningIndex\' must be in the range 0..N-1 where N is the number of"]
    #[doc = "warnings returned by \'pico_getNrEngineWarnings\'. \'outCode\' returns"]
    #[doc = "the warning as an integer code (cf. PICO_WARN_*)."]
    pub fn pico_getEngineWarning(
        engine: pico_Engine,
        warningIndex: pico_Int32,
        outCode: *mut pico_Status,
        outMessage: *mut ::std::os::raw::c_char,
    ) -> pico_Status;
}
