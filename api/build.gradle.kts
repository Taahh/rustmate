plugins {
    id("java")
}

group = "dev.taah"
version = "0.1"

repositories {
    mavenCentral()
}

dependencies {
    compileOnly("org.jetbrains:annotations:20.1.0")
    compileOnly("io.netty:netty-all:4.1.76.Final")
    compileOnly("org.projectlombok:lombok:1.18.24")
    annotationProcessor("org.projectlombok:lombok:1.18.24")
}