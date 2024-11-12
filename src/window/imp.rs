use std::fs;
use glib::subclass::InitializingObject;

use gtk::glib::clone;
use gtk::{prelude::*, Box, Label, ListBox, ListBoxRow};
use gtk::subclass::prelude::*;
use gtk::{glib, CompositeTemplate};
use nvml_wrapper::Nvml;
use sysinfo::{Components, Disks, System};

#[derive(CompositeTemplate, Default)]
#[template(resource = "/com/ning/sys-info-fetch/window.ui")]
pub struct Window {
    #[template_child]
    pub total_memory_label: TemplateChild<Label>,
    #[template_child]
    pub total_memory_value: TemplateChild<Label>,


    #[template_child]
    pub name_label_value: TemplateChild<Label>,
    #[template_child]
    pub kernel_version_value: TemplateChild<Label>,
    #[template_child]
    pub os_version_value: TemplateChild<Label>,
    #[template_child]
    pub disk_list_box: TemplateChild<ListBox>,
    #[template_child]
    pub sensor_list_box: TemplateChild<ListBox>,
    #[template_child]
    pub gpu_list_box: TemplateChild<ListBox>,
    
}

#[glib::object_subclass]
impl ObjectSubclass for Window {
    const NAME: &'static str = "MainWindow";
    type Type = super::Window;
    type ParentType = gtk::ApplicationWindow;

    fn class_init(klass: &mut Self::Class) {
        klass.bind_template();
    }

    fn instance_init(obj: &InitializingObject<Self>) {
        obj.init_template();
    }
}

impl ObjectImpl for Window {
    fn constructed(&self) {
        self.parent_constructed();

        //let now = std::time::Instant::now();
        let window = self;
        glib::spawn_future_local(clone!(
            #[weak]
            window,
            async move {
                let sys = System::new_all();
                let total_mem = (sys.total_memory() as f32) / (1024.0 * 1024.0 * 1024.0);
                let total_mem_str = format!("{:.2}", total_mem);
                let used_mem = (sys.used_memory() as f32) / (1024.0 * 1024.0 * 1024.0);
                let used_mem_str = format!("{:.2}", used_mem);
                let total_mem_str = "used ".to_string() + used_mem_str.as_str() + " of " + total_mem_str.as_str();
                window.total_memory_value.set_label((total_mem_str + " G").as_str());

                if let Some(name) = System::name(){
                    window.name_label_value.set_label(name.as_str());
                }else {
                    window.name_label_value.set_label("---");
                }
                if let Some(kernal) = System::kernel_version(){
                    window.kernel_version_value.set_label(kernal.as_str());
                }else {
                    window.kernel_version_value.set_label("---");
                }
                if let Some(osv) = System::os_version(){
                    window.os_version_value.set_label(osv.as_str());
                }else {
                    window.os_version_value.set_label("---");
                }
                
        
                let disks = Disks::new_with_refreshed_list();
        
                for disk in &disks {
                    let row = ListBoxRow::new();
                    let hbox = Box::new(gtk::Orientation::Horizontal, 5);
        
                    let ava_space = disk.available_space() / (1024 * 1024 * 1024);
                    let total_space = disk.total_space() / (1024 * 1024 * 1024);
                    let disk_str = ava_space.to_string() + "  available of " + total_space.to_string().as_str() + "G";
        
                    let key_label = Label::new(Some(&format!("{}", disk.name().to_string_lossy().to_string())));
                    let value_label = Label::new(Some(&format!("{}", disk_str)));
                    value_label.set_selectable(true);
                    hbox.append(&key_label);
                    hbox.append(&value_label);
                    hbox.set_homogeneous(true);
                    row.set_child(Some(&hbox));
                    
                    window.sensor_list_box.append(&row);
                }
        
                
                let components = Components::new_with_refreshed_list();
        
                for component in &components {
                    let row = ListBoxRow::new();
                    let hbox = Box::new(gtk::Orientation::Horizontal, 5);
                    let key_label = Label::new(Some(&format!("{}", component.label())));
                    let value_label = Label::new(Some(&format!("{} °C", component.temperature())));
                    value_label.set_selectable(true);
                    hbox.append(&key_label);
                    hbox.append(&value_label);
                    hbox.set_homogeneous(true);
                    row.set_child(Some(&hbox));
                    
                    window.sensor_list_box.append(&row);
                }
        
                let row = ListBoxRow::new();
                let hbox = Box::new(gtk::Orientation::Horizontal, 5);
                let key_label = Label::new(Some("cpu brand: "));
                let value_label = Label::new(Some(&format!("{}", &sys.cpus()[0].brand())));
                value_label.set_selectable(true);
                hbox.append(&key_label);
                hbox.append(&value_label);
                hbox.set_homogeneous(true);
                row.set_child(Some(&hbox));     
                window.gpu_list_box.append(&row);
        
                
                let row = ListBoxRow::new();
                let hbox = Box::new(gtk::Orientation::Horizontal, 5);
                let key_label = Label::new(Some("cpu numbers: "));
                let value_label = Label::new(Some(&format!("{}", &sys.cpus().len())));
                value_label.set_selectable(true);
                hbox.append(&key_label);
                hbox.append(&value_label);
                hbox.set_homogeneous(true);
                row.set_child(Some(&hbox));     
                window.gpu_list_box.append(&row);

                let path = format!("/sys/devices/system/cpu/cpu{}/cpufreq/scaling_cur_freq", 0);
                if let Ok(content) = fs::read_to_string(&path) {
                    if let Ok(freq) = content.trim().parse::<u64>() {
                        let row = ListBoxRow::new();
                        let hbox = Box::new(gtk::Orientation::Horizontal, 5);
                        let key_label = Label::new(Some("cpu freq: "));
                        let value_label = Label::new(Some(&format!("{} MHz", freq / 1000)));
                        value_label.set_selectable(true);
                        hbox.append(&key_label);
                        hbox.append(&value_label);
                        hbox.set_homogeneous(true);
                        row.set_child(Some(&hbox));     
                        window.gpu_list_box.append(&row);
                    }
                }
        
                let nvml = Nvml::init().expect("fail to init NVML");
                let device = nvml.device_by_index(0).expect("fail to get NVML info");
        
                let row = ListBoxRow::new();
                let hbox = Box::new(gtk::Orientation::Horizontal, 5);
                let key_label = Label::new(Some("gpu brand"));
                let name = device.name().expect("Failed to get device name");
                let value_label = Label::new(Some(&format!("{}", name)));
                value_label.set_selectable(true);
                hbox.append(&key_label);
                hbox.append(&value_label);
                hbox.set_homogeneous(true);
                row.set_child(Some(&hbox));     
                window.gpu_list_box.append(&row);
        
                let row = ListBoxRow::new();
                let hbox = Box::new(gtk::Orientation::Horizontal, 5);
                let memory_info = device.memory_info().expect("Failed to get memory info");
                let key_label = Label::new(Some("gpu mem"));
                let value_label = Label::new(Some(&format!("{} used of {} MB", memory_info.used / (1024 * 1024), memory_info.total / (1024 * 1024))));
                value_label.set_selectable(true);
                hbox.append(&key_label);
                hbox.append(&value_label);
                hbox.set_homogeneous(true);
                row.set_child(Some(&hbox));     
                window.gpu_list_box.append(&row);
        
                match device.temperature(nvml_wrapper::enum_wrappers::device::TemperatureSensor::Gpu) {
                    Ok(temp) => {
                        let row = ListBoxRow::new();
                        let hbox = Box::new(gtk::Orientation::Horizontal, 5);
                        let key_label = Label::new(Some("gpu temp"));
                        let value_label = Label::new(Some(&format!("{} °C", temp)));
                        value_label.set_selectable(true);
                        hbox.append(&key_label);
                        hbox.append(&value_label);
                        hbox.set_homogeneous(true);
                        row.set_child(Some(&hbox));     
                        window.gpu_list_box.append(&row);
                    },
                    Err(e) => eprintln!("retrieve gpu temp failed: {:?}", e),
                }
            }
        ));



        
        
        
        //println!("elaped::::{}",  now.elapsed().as_millis());
        
    }
}

impl WidgetImpl for Window {}

impl WindowImpl for Window {}

impl ApplicationWindowImpl for Window {}