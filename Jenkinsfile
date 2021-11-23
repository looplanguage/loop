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
                }
            }
        }
    }
}
