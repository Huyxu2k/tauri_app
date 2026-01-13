

pub const ADB: &str = "adb";

pub const DEVICE: &str = "device";

pub const DEVICES: &str = "devices";
    pub const l: &str = "-l"; // chi tiết thiết bị

pub const VERSION: &str = "version";

// copy từ máy tính -> điện thoại
pub const PUSH: &str = "push"; 
    // adb push file.txt /sdcard 

// copy từ điện thoại -> máy tính
pub const PULL: &str = "pull";

// phím cứng
pub const HOME: &str = "3";
pub const BACK: &str = "4";
pub const MENU: &str = "82";
pub const POWER: &str = "26";
pub const VOLUME_UP: &str = "24";
pub const VOLUME_DOWN: &str = "25";
pub const CAMERA: &str = "27";

// cài/gỡ ứng dụng
pub const INSTALL: &str = "install";
pub const UNINSTALL: &str = "uninstall";
    // adb install app.apk              # cài ứng dụng bằng file apk
    // adb install -r app.apk           # ghi đè ứng dụng đã cài
    // adb uninstall com.example.app    # gỡ ứng dụng theo tên

// điều khiển sâu
// ls
// cd
// pwd
// rm file.txt
// -r folder
// cp a.txt b.txt
// mv a.txt b.txt
pub const SHELL: &str = "shell";
    // adb shell pm list packages                   # danh sách ứng dụng
    // adb shell pm list packages | grep facebook   # kiếm tra xem có facebook trong danh sách ứng dụng không
    // adb shell pm uninstall --user 0 com.package.name gỡ app hệ thống

    // mô phỏng cảm ứng
    // adb shell input tap 500 1000 
    // adb shell input swipe 100 100 800 800
    // adb shell input swipe 100 800 100 100 300
    
    // nhập văn bản
    // adb shell input text hello_world
    // adb shell input keyevent 66         # enter

    // gõ phím cứng
    // adb shell input keyevent 3

    // điều khiển nguồn , màn hình
    // adb shell input keyevent 26   # Tắt/bật màn hình
    // adb shell reboot
    // adb shell reboot recovery
    // adb shell reboot bootloader
    // adb power off

    // chụp màn hình, quay video
    // adb shell screencap /sdcard/screen.png
    // adb pull /sdcard/screen.png 
    // adb shell screenrecord /sdcard/demo.mp4

    // log, debug , theo dõi
    // adb logcat
    // adb logcat -c     # Clear log
    // adb logcat | grep com.example

    // theo dõi CPU, RAM
    // adb shell top
    // adb shell dumpsys meminfo
    // adb shell dumpsys cpuinfo

    // quản lý tiến trình, dịch vụ
    // adb shell ps
    // adb shell kill 1234
    // adb shell am force-stop com.example.app


    // điều khiển Activity / Intent
    // adb shell monkey -p com.example.app 1                                        # mỡ ứng dụng
    // adb shell am start -n com.example/.MainActivity                              # mở activity cụ thể
    // adb shell am start -a android.intent.action.VIEW -d https://google.com       # mở url cụ thể
    // adb shell am broadcast -a ACTION_NAME                                        # gửi broadcast   


    // mạng , wifi, dữ liệu
    // adb shell svc wifi enable        # bật/tắt wifi
    // adb shell svc wifi disable   
    // adb shell svc data enable        # bật/tắt dữ liệu
    // adb shell svc data disable
    // adb shell ip addr show wlan0     # xem ip

    // pin , giả lập trạng thái pin
    // adb shell dumpsys battery
    // adb shell dumpsys battery set level 50
    // adb shell dumpsys battery set status 2
    // adb shell dumpsys battery reset


    // quyền root (nếu có)
    // adb root
    // adb remount
    // adb shell su
    
    // backup, restore
    // adb backup -apk -shared -all -f backup.ab
    // adb restore backup.ab

    // thông tin hệ thống
    // adb shell getprop
    // adb shell getprop ro.product.model
    // adb shell getprop ro.build.version.release
    // adb shell uname -a
    
    
