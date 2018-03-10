pipeline {
    agent {
        table 'rust'
    }

    stages {
        stage('Build') {
            steps {
                sh "cargo build --release"
            }
        }
        stage('Rustfmt') {
            steps {
                // The build will fail if rustfmt thinks any changes are
                // required.
                sh "cargo +nightly fmt --all -- --write-mode diff"
            }
        }
    }
}
