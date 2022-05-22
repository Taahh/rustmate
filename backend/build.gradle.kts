plugins {
    id("java")
}

group = "dev.taah"
version = "0.1"

repositories {
    mavenCentral()
}

dependencies {
    implementation("com.google.code.gson:gson:2.9.0")
    compileOnly(project(":api"))
    compileOnly("io.netty:netty-all:4.1.77.Final")
    compileOnly("org.projectlombok:lombok:1.18.24");
    compileOnly("com.google.guava:guava:31.1-jre")
    compileOnly("org.apache.commons:commons-lang3:3.12.0")
    annotationProcessor("org.projectlombok:lombok:1.18.24");
}

tasks.getByName<Test>("test") {
    useJUnitPlatform()
}