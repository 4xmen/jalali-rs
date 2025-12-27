# Jalali-rs

![License](https://img.shields.io/badge/license-MIT-blue.svg) 
[![Docs Status](https://docs.rs/jalali-rs/badge.svg)](https://docs.rs/jalali-rs)
[![Latest Version](https://img.shields.io/crates/v/jalali-rs.svg)](https://crates.io/crates/jalali-rs)



A lightweight and efficient Rust crate for converting dates between the Gregorian and Jalali (Persian) calendars. This library provides accurate date conversions, support for Unix timestamps, string parsing with custom separators, and digit conversions between Latin, Persian, and Arabic numerals. It's designed to be simple, dependency-free, and easy to integrate into your Rust projects.

No external dependencies are used, ensuring compatibility and ease of use across environments. All calculations are performed with i64 to prevent overflows, and functions return `Option` where appropriate to handle invalid inputs gracefully without panics.

## Features

- **Gregorian to Jalali Conversion**: Convert Gregorian dates to Jalali dates.
- **Jalali to Gregorian Conversion**: Convert Jalali dates back to Gregorian.
- **Unix Timestamp Support**: Convert Unix timestamps to Jalali dates and vice versa (assuming UTC midnight).
- **String Parsing and Formatting**: Parse date strings with custom separators, handling Persian/Arabic digits automatically, and output formatted strings.
- **Digit Conversions**: Convert between Latin, Persian, and Arabic digits for flexible input handling.
- **Robust Error Handling**: Returns `Option` for potentially invalid operations to avoid runtime panics.
- **Comprehensive Documentation**: Each function includes detailed doc comments with examples.
- **Tests Included**: Unit tests for all core functionalities to ensure reliability.

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
jalali-rs = "0.1.0"  # Replace with the latest version
```

Then, run `cargo build` to fetch and compile the crate.

## Usage

Here's a quick overview of how to use the library. For more details, check the [API documentation](https://docs.rs/jalali-rs).

### Basic Date Conversion

```rust
use jalali_rs::{gregorian_to_jalali, jalali_to_gregorian};

let (jy, jm, jd) = gregorian_to_jalali(2025, 12, 27);
println!("Jalali: {}-{}-{}", jy, jm, jd);  // Output: Jalali: 1404-10-6

let (gy, gm, gd) = jalali_to_gregorian(1404, 10, 6);
println!("Gregorian: {}-{}-{}", gy, gm, gd);  // Output: Gregorian: 2025-12-27
```

### Unix Timestamp Conversion

```rust
use jalali_rs::{unix_to_jalali, jalali_to_unix};

if let Some((jy, jm, jd)) = unix_to_jalali(0) {
    println!("Jalali from Unix 0: {}-{}-{}", jy, jm, jd);  // Output: 1348-10-11
}

if let Some(timestamp) = jalali_to_unix(1348, 10, 11) {
    println!("Unix from Jalali: {}", timestamp);  // Output: 0
}
```

### String Parsing with Digit Handling

```rust
use jalali_rs::{parse_gregorian_string_to_jalali_string, parse_jalali_string_to_gregorian_string};

let jalali_str = parse_gregorian_string_to_jalali_string("2025-12-27", '-');
println!("{:?}", jalali_str);  // Some("1404-10-06")

let gregorian_str = parse_jalali_string_to_gregorian_string("۱۴۰۴/۱۰/۰۶", '/');  // Handles Persian digits
println!("{:?}", gregorian_str);  // Some("2025-12-27")
```

### Digit Conversions

```rust
use jalali_rs::{latin_digits_to_persian, persian_or_arabic_digits_to_latin};

let persian = latin_digits_to_persian("1400-12-10");
println!("{}", persian);  // ۱۴۰۰-۱۲-۱۰

let latin = persian_or_arabic_digits_to_latin("١٤٠٠-١٢-١٠");  // Handles Arabic digits
println!("{}", latin);  // 1400-12-10
```

## Documentation

Full API documentation is available on [docs.rs/jalali-rs](https://docs.rs/jalali-rs). Each function includes examples and detailed explanations.

## Contributing

Contributions are welcome! Please feel free to submit pull requests for bug fixes, new features, or improvements. Make sure to add tests for any new functionality.

1. Fork the repository.
2. Create a new branch: `git checkout -b feature-branch`.
3. Commit your changes: `git commit -m 'Add some feature'`.
4. Push to the branch: `git push origin feature-branch`.
5. Submit a pull request.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Acknowledgments

- Based on proven algorithms for Gregorian-Jalali conversions. 
- Inspired by community needs for a simple, no-dependency Persian calendar library in Rust.
- With special thanks to [scr-ir](https://github.com/scr-ir]) — this package's core algorithm is inspired by his original open-source algorithm.
---

# Jalali-rs (جلالی-آراس)

![License](https://img.shields.io/badge/license-MIT-blue.svg)
[![Docs Status](https://docs.rs/jalali-rs/badge.svg)](https://docs.rs/jalali-rs)
[![Latest Version](https://img.shields.io/crates/v/jalali-rs.svg)](https://crates.io/crates/jalali-rs)


یک کرات سبک و کارآمد در Rust برای تبدیل تاریخ‌های میلادی (گریگوری) به شمسی (جلالی) و برعکس. این کتابخانه تبدیل‌های دقیق تاریخ، پشتیبانی از تایم‌استمپ یونیکس، پارس رشته‌های تاریخ با جداکننده‌های سفارشی، و تبدیل اعداد بین لاتین، پارسی و عربی را فراهم می‌کند. طراحی شده تا ساده، بدون وابستگی خارجی، و آسان برای ادغام در پروژه‌های Rust شما باشد.

هیچ وابستگی خارجی استفاده نشده، که سازگاری و سهولت استفاده در محیط‌های مختلف را تضمین می‌کند. تمام محاسبات با i64 انجام می‌شود تا از اورفلو جلوگیری شود، و توابع در جایی که لازم است `Option` برمی‌گردانند تا ورودی‌های نامعتبر را بدون پنیک مدیریت کنند.

## ویژگی‌ها

- **تبدیل میلادی به شمسی**: تبدیل تاریخ‌های میلادی به شمسی.
- **تبدیل شمسی به میلادی**: تبدیل تاریخ‌های شمسی به میلادی.
- **پشتیبانی از تایم‌استمپ یونیکس**: تبدیل تایم‌استمپ یونیکس به تاریخ شمسی و برعکس (با فرض نیمه‌شب UTC).
- **پارس و فرمت رشته‌ها**: پارس رشته‌های تاریخ با جداکننده‌های سفارشی، مدیریت خودکار اعداد پارسی/عربی، و خروجی رشته‌های فرمت‌شده.
- **تبدیل اعداد**: تبدیل بین اعداد لاتین، پارسی و عربی برای مدیریت ورودی‌های انعطاف‌پذیر.
- **مدیریت خطاهای قوی**: برگرداندن `Option` برای عملیات‌های بالقوه نامعتبر برای جلوگیری از پنیک‌های زمان اجرا.
- **داکیومنتاسیون جامع**: هر تابع شامل کامنت‌های دقیق با مثال‌ها.
- **تست‌های شامل**: تست‌های واحد برای تمام عملکردهای اصلی برای اطمینان از قابلیت اعتماد.

## نصب

این را به `Cargo.toml` خود اضافه کنید:

```toml
[dependencies]
jalali-rs = "0.1.0"  # نسخه آخرین را جایگزین کنید
```

سپس، `cargo build` را اجرا کنید تا کرات دانلود و کامپایل شود.

## استفاده

در اینجا مروری سریع بر نحوه استفاده از کتابخانه آورده شده است. برای جزئیات بیشتر، [داکیومنتاسیون API](https://docs.rs/jalali-rs) را بررسی کنید.

### تبدیل پایه تاریخ

```rust
use jalali_rs::{gregorian_to_jalali, jalali_to_gregorian};

let (jy, jm, jd) = gregorian_to_jalali(2025, 12, 27);
println!("شمسی: {}-{}-{}", jy, jm, jd);  // خروجی: شمسی: 1404-10-6

let (gy, gm, gd) = jalali_to_gregorian(1404, 10, 6);
println!("میلادی: {}-{}-{}", gy, gm, gd);  // خروجی: میلادی: 2025-12-27
```

### تبدیل تایم‌استمپ یونیکس

```rust
use jalali_rs::{unix_to_jalali, jalali_to_unix};

if let Some((jy, jm, jd)) = unix_to_jalali(0) {
    println!("شمسی از یونیکس ۰: {}-{}-{}", jy, jm, jd);  // خروجی: 1348-10-11
}

if let Some(timestamp) = jalali_to_unix(1348, 10, 11) {
    println!("یونیکس از شمسی: {}", timestamp);  // خروجی: ۰
}
```

### پارس رشته با مدیریت اعداد

```rust
use jalali_rs::{parse_gregorian_string_to_jalali_string, parse_jalali_string_to_gregorian_string};

let jalali_str = parse_gregorian_string_to_jalali_string("2025-12-27", '-');
println!("{:?}", jalali_str);  // Some("1404-10-06")

let gregorian_str = parse_jalali_string_to_gregorian_string("۱۴۰۴/۱۰/۰۶", '/');  // مدیریت اعداد پارسی
println!("{:?}", gregorian_str);  // Some("2025-12-27")
```

### تبدیل اعداد

```rust
use jalali_rs::{latin_digits_to_persian, persian_or_arabic_digits_to_latin};

let persian = latin_digits_to_persian("1400-12-10");
println!("{}", persian);  // ۱۴۰۰-۱۲-۱۰

let latin = persian_or_arabic_digits_to_latin("١٤٠٠-١٢-١٠");  // مدیریت اعداد عربی
println!("{}", latin);  // 1400-12-10
```

## داکیومنتاسیون

داکیومنتاسیون کامل API در [docs.rs/jalali-rs](https://docs.rs/jalali-rs) موجود است. هر تابع شامل توضیحات دقیق و مثال‌ها است.

## مشارکت

مشارکت‌ها خوش‌آمد هستند! لطفاً برای رفع باگ‌ها، ویژگی‌های جدید یا بهبودها، pull request ارسال کنید. مطمئن شوید که برای عملکردهای جدید تست اضافه کنید.

۱. مخزن را فورک کنید.
۲. شاخه جدید ایجاد کنید: `git checkout -b feature-branch`.
۳. تغییرات را کامیت کنید: `git commit -m 'Add some feature'`.
۴. به شاخه push کنید: `git push origin feature-branch`.
۵. pull request ارسال کنید.

## لایسنس

این پروژه تحت لایسنس MIT است - فایل [LICENSE](LICENSE) را برای جزئیات ببینید.

## قدردانی

- بر اساس الگوریتم‌های اثبات‌شده برای تبدیل‌های گریگوری-جلالی.
- الهام‌گرفته از نیازهای جامعه برای یک کتابخانه ساده تقویم پارسی بدون وابستگی در Rust.
- با قدردانی ویژه از  [scr-ir](https://github.com/scr-ir]) بابت الگورتیم پایه این پکیج الهام گرفته از الگورتیم ایشان