pipeline {
    agent { label 'windows' }

    stages {
        stage('Check') {
            steps {
                // Run check
                powershell 'cargo check'
            }
        }

        stage('Test') {
            steps {
                // Run tests
                powershell 'cargo test'
            }
        }

        stage('Build') {
            steps {
                // Do actual build
                powershell 'cargo build --release'
            }

            post {
                success {
                    archiveArtifacts 'target/release/*.exe'
                    s3Upload consoleLogLevel: 'INFO', dontSetBuildResultOnFailure: false, dontWaitForConcurrentBuildCompletion: false, entries: [[bucket: 'loopartifacts/${JOB_NAME}-${BUILD_NUMBER}', excludedFile: '', flatten: false, gzipFiles: false, keepForever: false, managedArtifacts: true, noUploadOnFailure: true, selectedRegion: 'us-east-2', showDirectlyInBrowser: false, sourceFile: 'target/release/*.exe', storageClass: 'STANDARD', uploadFromSlave: false, useServerSideEncryption: false]], pluginFailureResultConstraint: 'FAILURE', profileName: 'jenkins', userMetadata: []
                }
            }
        }
    }
}
