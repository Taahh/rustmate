plugins {
    java
}

dependencies {
    implementation("io.netty:netty-all:4.1.76.Final")
    implementation("org.jetbrains:annotations:20.1.0")
    implementation("com.google.guava:guava:31.1-jre")
    implementation("org.projectlombok:lombok:1.18.22")
    implementation(project(":backend"))
    implementation(project(":api"))
    annotationProcessor("org.projectlombok:lombok:1.18.22")
}