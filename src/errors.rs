error_chain! {

    foreign_links {
        SysfsGpio(::sysfs_gpio::Error);
    }
}
