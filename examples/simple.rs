use std::{ ffi::{ CStr, CString }, os::raw::{ c_char, c_int } };

use edflib_sys::*;

const SAMPLES_READ: usize = 200;
const CHANNEL: i32 = 1;

fn char_to_str(ptr: *mut i8) -> String {
    let cstr = unsafe { CStr::from_ptr(ptr) };
    let result = cstr.to_str().unwrap().to_owned().to_string();
    result
}

fn empty_char() -> [i8; 81] {
    let result: [i8; 81] = (0..81)
        .map(|_| 0)
        .collect::<Vec<_>>()
        .try_into()
        .expect("wrong size iterator");

    result
}

fn default_edf_param_struct() -> edf_param_struct {
    let result = edf_param_struct {
        label: Default::default(),
        smp_in_file: Default::default(),
        phys_max: Default::default(),
        phys_min: Default::default(),
        dig_max: Default::default(),
        dig_min: Default::default(),
        smp_in_datarecord: Default::default(),
        physdimension: Default::default(),
        prefilter: empty_char(),
        transducer: empty_char(),
    };

    result
}

fn empty_signalparam() -> [edflib_param_t; 4096] {
    let result: [edflib_param_t; 4096] = (0..4096)
        .map(|_| default_edf_param_struct())
        .collect::<Vec<_>>()
        .try_into()
        .expect("wrong size iterator");

    result
}

fn default_hdr() -> edf_hdr_struct {
    let hdr = edf_hdr_struct {
        handle: Default::default(),
        filetype: Default::default(),
        edfsignals: Default::default(),
        file_duration: Default::default(),
        startdate_day: Default::default(),
        startdate_month: Default::default(),
        startdate_year: Default::default(),
        starttime_subsecond: Default::default(),
        starttime_second: Default::default(),
        starttime_minute: Default::default(),
        starttime_hour: Default::default(),
        patient: empty_char(),
        recording: empty_char(),
        patientcode: empty_char(),
        sex: Default::default(),
        gender: Default::default(),
        birthdate: Default::default(),
        birthdate_day: Default::default(),
        birthdate_month: Default::default(),
        birthdate_year: Default::default(),
        patient_name: empty_char(),
        patient_additional: empty_char(),
        admincode: empty_char(),
        technician: empty_char(),
        equipment: empty_char(),
        recording_additional: empty_char(),
        datarecord_duration: Default::default(),
        datarecords_in_file: Default::default(),
        annotations_in_file: Default::default(),
        signalparam: empty_signalparam(),
    };

    hdr
}

pub fn main() {
    // let hdr = unsafe { MaybeUninit::<*mut edf_hdr_struct>::zeroed().assume_init() };
    // let hdr = unsafe { MaybeUninit::uninit().assume_init() };
    // let hdr = unsafe { MaybeUninit::uninit().assume_init() };
    let mut hdr = default_hdr();

    let path: *const c_char = CString::new("./generator.edf").unwrap().into_raw();

    println!("edflib-sys version {}", edflib_sys::EDFLIBSYS_VERSION.unwrap());
    let version = unsafe { edflib_sys::edflib_version() };
    println!("edflib version {}", (version as f64) / (100 as f64));

    // println!("usage: test_edflib <file> <signal nr>");

    let mut channel = CHANNEL;
    if channel < 1 {
        return println!("signalnumber must be > 0");
    }

    let open_file_result = unsafe {
        edfopen_file_readonly(path, &mut hdr, EDFLIB_READ_ALL_ANNOTATIONS as c_int)
    };

    if open_file_result > 0 {
        match hdr.filetype {
            EDFLIB_MALLOC_ERROR => println!("malloc error"),
            EDFLIB_NO_SUCH_FILE_OR_DIRECTORY =>
                println!("cannot open file, no such file or directory"),
            EDFLIB_FILE_CONTAINS_FORMAT_ERRORS =>
                println!("the file is not EDF(+) or BDF(+) complian (it contains format errors)"),
            EDFLIB_MAXFILES_REACHED => println!("to many files opened"),
            EDFLIB_FILE_READ_ERROR => println!("a read error occurred"),
            EDFLIB_FILE_ALREADY_OPENED => println!("file has already been opened"),
            _ => println!("unknown error"),
        }

        return;
    }

    let hdl = hdr.handle;

    if channel > hdr.edfsignals {
        println!("error: file has {} signals and you selected signal {}", hdr.edfsignals, channel);
        unsafe {
            edfclose_file(hdl);
        }
        return;
    }

    channel -= 1;

    println!("general header:");

    println!("filetype: {}", hdr.filetype);
    println!("edfsignals: {}", hdr.edfsignals);
    #[cfg(target_os = "windows")]
    println!("file duration: {} seconds", hdr.file_duration / (EDFLIB_TIME_DIMENSION as i64));

    #[cfg(not(target_os = "windows"))]
    println!("file duration: {} seconds", hdr.file_duration / (EDFLIB_TIME_DIMENSION as i64));

    println!("startdate: {}-{}-{}", hdr.startdate_day, hdr.startdate_month, hdr.startdate_year);

    #[cfg(target_os = "windows")]
    println!(
        "starttime: {}:{:02}:{:02}.{:07}",
        hdr.starttime_hour,
        hdr.starttime_minute,
        hdr.starttime_second,
        hdr.starttime_subsecond
    );
    #[cfg(not(target_os = "windows"))]
    println!(
        "starttime: {}:{:02}:{:02}.{:07}",
        hdr.starttime_hour,
        hdr.starttime_minute,
        hdr.starttime_second,
        hdr.starttime_subsecond
    );

    println!("patient: {}", char_to_str(hdr.patient.as_mut_ptr()));
    println!("recording: {}", char_to_str(hdr.recording.as_mut_ptr()));
    println!("patientcode: {}", char_to_str(hdr.patientcode.as_mut_ptr()));
    println!("sex: {}", char_to_str(hdr.sex.as_mut_ptr()));
    println!("birthdate: {}", char_to_str(hdr.birthdate.as_mut_ptr()));
    println!("patient_name: {}", char_to_str(hdr.patient_name.as_mut_ptr()));
    println!("patient_additional: {}", char_to_str(hdr.patient_additional.as_mut_ptr()));
    println!("admincode: {}", char_to_str(hdr.admincode.as_mut_ptr()));
    println!("technician: {}", char_to_str(hdr.technician.as_mut_ptr()));
    println!("equipment: {}", char_to_str(hdr.equipment.as_mut_ptr()));
    println!("recording_additional: {}", char_to_str(hdr.recording_additional.as_mut_ptr()));
    println!(
        "datarecord duration: {} seconds",
        hdr.datarecord_duration / (EDFLIB_TIME_DIMENSION as i64)
    );

    #[cfg(target_os = "windows")]
    {
        println!("number of datarecords in the file: {}", hdr.datarecords_in_file);
        println!("number of annotations in the file: {}", hdr.annotations_in_file);
    }

    #[cfg(not(target_os = "windows"))]
    {
        println!("number of datarecords in the file: {}", hdr.datarecords_in_file);
        println!("number of annotations in the file: {}", hdr.annotations_in_file);
    }

    println!("signal parameters:");

    println!("label: {}", char_to_str(hdr.signalparam[channel as usize].label.as_mut_ptr()));
    #[cfg(target_os = "windows")]
    println!("samples in file: {}", hdr.signalparam[channel as usize].smp_in_file);
    #[cfg(not(target_os = "windows"))]
    println!("samples in file: {}", hdr.signalparam[channel as usize].smp_in_file);

    println!("samples in datarecord: {}", hdr.signalparam[channel as usize].smp_in_datarecord);
    println!("physical maximum: {}", hdr.signalparam[channel as usize].phys_max);
    println!("physical minimum: {}", hdr.signalparam[channel as usize].phys_min);
    println!("digital maximum: {}", hdr.signalparam[channel as usize].dig_max);
    println!("digital minimum: {}", hdr.signalparam[channel as usize].dig_min);
    println!(
        "physical dimension: {}",
        char_to_str(hdr.signalparam[channel as usize].physdimension.as_mut_ptr())
    );
    println!(
        "prefilter: {}",
        char_to_str(hdr.signalparam[channel as usize].prefilter.as_mut_ptr())
    );
    println!(
        "transducer: {}",
        char_to_str(hdr.signalparam[channel as usize].transducer.as_mut_ptr())
    );
    println!(
        "samplefrequency: {}",
        (hdr.signalparam[channel as usize].smp_in_datarecord / (hdr.datarecord_duration as i32)) *
            (EDFLIB_TIME_DIMENSION as i32)
    );

    let duration: [i8; 20] = (0..20)
        .map(|_| 0)
        .collect::<Vec<_>>()
        .try_into()
        .expect("wrong size iterator");
    let annotation: [i8; (EDFLIB_MAX_ANNOTATION_LEN + 1) as usize] = (0..EDFLIB_MAX_ANNOTATION_LEN +
        1)
        .map(|_| 0)
        .collect::<Vec<_>>()
        .try_into()
        .expect("wrong size iterator");

    let mut annot: edflib_annotation_t = edf_annotation_struct {
        onset: 0,
        duration_l: 0,
        duration,
        annotation,
    };

    for i in 0..hdr.annotations_in_file {
        if (unsafe { edf_get_annotation(hdl, i as i32, &mut annot) }) > 0 {
            println!("error: edf_get_annotations()");
            unsafe {
                edfclose_file(hdl);
            }
            return;
        } else {
            #[cfg(target_os = "windows")]
            println!(
                "annotation: onset is {}    duration is {}    description is {}",
                annot.onset / EDFLIB_TIME_DIMENSION,
                annot.duration,
                annot.annotation
            );
            #[cfg(not(target_os = "windows"))]
            println!(
                "annotation: onset is {}.{:07} sec    duration is {}    description is \"{}\"",
                annot.onset / (EDFLIB_TIME_DIMENSION as i64),
                annot.onset % (EDFLIB_TIME_DIMENSION as i64),
                char_to_str(annot.duration.as_mut_ptr()),
                char_to_str(annot.annotation.as_mut_ptr())
            );
        }
    }

    //   buf = (double *)malloc(sizeof(double[SAMPLES_READ]));
    let mut buf: [f64; SAMPLES_READ] = (0..SAMPLES_READ)
        .map(|_| 0.0)
        .collect::<Vec<_>>()
        .try_into()
        .expect("wrong size iterator");

    //   if(buf==NULL)
    //   {
    //     println!("malloc error");
    //     unsafe { edfclose_file(hdl) };
    //     return(1);
    //   }

    let x = 10; /* start reading x seconds from start of file */

    unsafe {
        edfseek(
            hdl,
            channel,
            (x / (hdr.file_duration / (EDFLIB_TIME_DIMENSION as i64))) *
                hdr.signalparam[channel as usize].smp_in_file,
            EDFSEEK_SET as i32
        );
    }

    let n = unsafe {
        edfread_physical_samples(hdl, channel, SAMPLES_READ as i32, buf.as_mut_ptr())
    };

    if n == -1 {
        println!("error: edf_read_physical_samples()");
        unsafe {
            edfclose_file(hdl);
        }
        return;
    }

    println!("read {} samples, started at {} seconds from start of file:", n, x);

    for i in 0..n {
        print!("{:.0}   ", buf[i as usize]);
    }
    unsafe {
        edfclose_file(hdl);
    }
}
