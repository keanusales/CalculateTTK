import java.util.Properties
import java.io.FileInputStream

plugins {
    id("com.android.application")
    id("org.jetbrains.kotlin.plugin.compose")
}

android {
    namespace = "com.delta.ttk"
    compileSdk = 36

    defaultConfig {
        applicationId = "com.delta.ttk"
        minSdk = 26
        targetSdk = 36
        versionCode = 1
        versionName = "1.0"
        resourceConfigurations.addAll(listOf("en"))
    }
    buildFeatures {
        compose = true
    }

    signingConfigs {
        create("release") {
            val keystoreProperties = Properties()
            val keystorePropertiesFile = rootProject.file("local.properties")
            
            if (keystorePropertiesFile.exists()) {
                keystoreProperties.load(FileInputStream(keystorePropertiesFile))
            }
            
            storeFile = file("../delta-release-key.jks")
            storePassword = keystoreProperties.getProperty("KEYSTORE_PASSWORD") ?: ""
            keyAlias = keystoreProperties.getProperty("KEY_ALIAS") ?: ""
            keyPassword = keystoreProperties.getProperty("KEY_PASSWORD") ?: ""
        }
    }

    buildTypes {
        release {
            isMinifyEnabled = true
            isShrinkResources = true
            proguardFiles(getDefaultProguardFile("proguard-android-optimize.txt"), "proguard-rules.pro")
            signingConfig = signingConfigs.getByName("release")
        }
    }

    compileOptions {
        sourceCompatibility = JavaVersion.VERSION_21
        targetCompatibility = JavaVersion.VERSION_21
    }
}

dependencies {
    implementation("androidx.core:core-ktx:1.13.1")
    implementation("androidx.activity:activity-compose:1.9.0")
    implementation(platform("androidx.compose:compose-bom:2024.04.01"))
    implementation("androidx.compose.ui:ui")
    implementation("androidx.compose.material3:material3")
}