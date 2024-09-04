use std::{ ffi::CString, os::raw::c_char };

use edflib_sys::*;

pub fn main() {
    let mut hdr: edf_hdr_struct;

    let path: *const c_char = CString::new("generator.edf").unwrap().into_raw();

    println!("edflib-sys version {}", edflib_sys::EDFLIBSYS_VERSION.unwrap());
    let version = unsafe { edflib_sys::edflib_version() };
    println!("edflib version {}", (version as f64) / (100 as f64));


    // println!("\nusage: test_edflib <file> <signal nr>\n\n");

  let mut channel = 1;
  if(channel<1)
  {
    return println!("\nsignalnumber must be > 0\n");
  }

  if unsafe {edfopen_file_readonly(path, &mut hdr, EDFLIB_READ_ALL_ANNOTATIONS as i32)} > 0
  {
    match hdr.filetype
    {
        EDFLIB_MALLOC_ERROR                =>  println!("\nmalloc error\n"),
        EDFLIB_NO_SUCH_FILE_OR_DIRECTORY   => println!("cannot open file, no such file or directory\n"),
        EDFLIB_FILE_CONTAINS_FORMAT_ERRORS => println!("the file is not EDF(+) or BDF(+) compliant\n(it contains format errors)\n"),
        EDFLIB_MAXFILES_REACHED            =>  println!("\nto many files opened\n"),
        EDFLIB_FILE_READ_ERROR             =>  println!("\na read error occurred\n"),
        EDFLIB_FILE_ALREADY_OPENED         =>  println!("\nfile has already been opened\n"),
        _                                 =>  println!("\nunknown error\n")
    }

    return;
  }

  let hdl = hdr.handle;

  if(channel>(hdr.edfsignals))
  {
    println!("\nerror: file has %i signals and you selected signal %i\n", hdr.edfsignals, channel);
    edfclose_file(hdl);
    return;
  }

  channel-=1;

  println!("\nlibrary version: %i.%02i\n", edflib_version() / 100, edflib_version() % 100);

  println!("\ngeneral header:\n\n");

  println!("filetype: %i\n", hdr.filetype);
  println!("edfsignals: %i\n", hdr.edfsignals);
  #[cfg(target_os = "windows")]
  println!("file duration: %I64d seconds\n", hdr.file_duration / EDFLIB_TIME_DIMENSION);

  #[cfg(not(target_os = "windows"))]
  println!("file duration: %lli seconds\n", hdr.file_duration / EDFLIB_TIME_DIMENSION);

  println!("startdate: %i-%i-%i\n", hdr.startdate_day, hdr.startdate_month, hdr.startdate_year);
  
  #[cfg(target_os = "windows")]
  println!("starttime: %i:%02i:%02i.%07I64d\n", hdr.starttime_hour, hdr.starttime_minute, hdr.starttime_second, hdr.starttime_subsecond);
  #[cfg(not(target_os = "windows"))]
  println!("starttime: %i:%02i:%02i.%07lli\n", hdr.starttime_hour, hdr.starttime_minute, hdr.starttime_second, hdr.starttime_subsecond);

  println!("patient: %s\n", hdr.patient);
  println!("recording: %s\n", hdr.recording);
  println!("patientcode: %s\n", hdr.patientcode);
  println!("sex: %s\n", hdr.sex);
  println!("birthdate: %s\n", hdr.birthdate);
  println!("patient_name: %s\n", hdr.patient_name);
  println!("patient_additional: %s\n", hdr.patient_additional);
  println!("admincode: %s\n", hdr.admincode);
  println!("technician: %s\n", hdr.technician);
  println!("equipment: %s\n", hdr.equipment);
  println!("recording_additional: %s\n", hdr.recording_additional);
  println!("datarecord duration: %f seconds\n", ((double)hdr.datarecord_duration) / EDFLIB_TIME_DIMENSION);

  #[cfg(target_os = "windows")]{
  println!("number of datarecords in the file: %I64d\n", hdr.datarecords_in_file);
  println!("number of annotations in the file: %I64d\n", hdr.annotations_in_file);
  }

  #[cfg(not(target_os = "windows"))]{
  println!("number of datarecords in the file: %lli\n", hdr.datarecords_in_file);
  println!("number of annotations in the file: %lli\n", hdr.annotations_in_file);
  }

  println!("\nsignal parameters:\n\n");

  println!("label: %s\n", hdr.signalparam[channel].label);
  #[cfg(target_os = "windows")]
  println!("samples in file: %I64d\n", hdr.signalparam[channel].smp_in_file);
  #[cfg(not(target_os = "windows"))]
  println!("samples in file: %lli\n", hdr.signalparam[channel].smp_in_file);

  println!("samples in datarecord: %i\n", hdr.signalparam[channel].smp_in_datarecord);
  println!("physical maximum: %f\n", hdr.signalparam[channel].phys_max);
  println!("physical minimum: %f\n", hdr.signalparam[channel].phys_min);
  println!("digital maximum: %i\n", hdr.signalparam[channel].dig_max);
  println!("digital minimum: %i\n", hdr.signalparam[channel].dig_min);
  println!("physical dimension: %s\n", hdr.signalparam[channel].physdimension);
  println!("prefilter: %s\n", hdr.signalparam[channel].prefilter);
  println!("transducer: %s\n", hdr.signalparam[channel].transducer);
  println!("samplefrequency: %f\n", ((double)hdr.signalparam[channel].smp_in_datarecord / (double)hdr.datarecord_duration) * EDFLIB_TIME_DIMENSION);

  let mut annot: edflib_annotation_t;

  for i in 0..hdr.annotations_in_file
  {
    if(unsafe { edf_get_annotation(hdl, i, &annot) })
    {
      println!("\nerror: edf_get_annotations()\n");
      unsafe { edfclose_file(hdl) };
      return(1);
    }
    else
    {
printf(
      println!("annotation: onset is %I64d    duration is %s    description is %s\n",
            annot.onset / EDFLIB_TIME_DIMENSION,
            annot.duration,
            annot.annotation);
#else
      println!("annotation: onset is %lli.%07lli sec    duration is %s    description is \"%s\"\n",
            annot.onset / EDFLIB_TIME_DIMENSION,
            annot.onset % EDFLIB_TIME_DIMENSION,
            annot.duration,
            annot.annotation);
#endif
    }
  }

  buf = (double *)malloc(sizeof(double[SAMPLES_READ]));
  if(buf==NULL)
  {
    println!("\nmalloc error\n");
    edfclose_file(hdl);
    return(1);
  }

  let x=10; /* start reading x seconds from start of file */

  edfseek(hdl, channel, (long long)((((double)x) / ((double)hdr.file_duration / (double)EDFLIB_TIME_DIMENSION)) * ((double)hdr.signalparam[channel].smp_in_file)), EDFSEEK_SET);

  n = edfread_physical_samples(hdl, channel, SAMPLES_READ, buf);

  if(n==(-1))
  {
    println!("error: edf_read_physical_samples()");
    edfclose_file(hdl);
    drop(buf);
    return;
  }

  println("read %i samples, started at %i seconds from start of file:\n", n, x);

  for i in 0..n
  {
    print!("%.0f  ", buf[i]);
  }
  edfclose_file(hdl);
  drop(buf);

}
