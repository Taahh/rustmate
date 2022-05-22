plugins {
    java
}

allprojects {
    group = "dev.taah"
    version = "0.1"
}

subprojects {
    apply(plugin = "java")
    repositories {
        mavenCentral()
    }
}