/*
 * SPDX-FileCopyrightText: 2023 UnionTech Software Technology Co., Ltd.
 *
 * SPDX-License-Identifier: MulanPSL-2.0
 */

fn main() {
    // 搜索本地动态库
    println!("cargo:rustc-link-search=native=./lib/util/.libs/");
    println!("cargo:rustc-link-search=native=/usr/lib64/");

    //链接.o文件
    //    println!("cargo:rust-link-search=");
}
