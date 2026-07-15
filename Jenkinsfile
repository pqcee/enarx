pipeline {
    agent {
        dockerfile {
          filename 'Dockerfile.build'
        }
    }

    environment {
        AZURE_CONFIG_DIR = "${env.WORKSPACE}/.azure"

    }

    stages {

        stage('Build Library') {
            steps {
                sh '. /usr/local/cargo/env && cargo build --verbose --release --locked'
            }
        }

        stage('Get Git Tag') {
            steps {
                script {
                    // Retrieve the latest Git tag
                    GIT_TAG = sh(script: 'git describe --tags', returnStdout: true).trim()
                    echo "Git Tag: ${GIT_TAG}"
                }
            }
        }

        stage('Build and Package Artifacts') {
            steps {
                script {
                    // Use Groovy to pass the GIT_TAG variable to the shell script
                    sh """
                    mkdir -p artifacts

                    # Copy the required files into the artifact directory
                    cp target/release/enarx artifacts/

                    # Package the directory into a tarball with the Git tag in the filename
                    tar -czvf enarx.tar.gz -C artifacts enarx
                    """
                }
            }
        }

        stage('Upload to Azure Blob Storage') {
            steps {
                withCredentials([
                    usernamePassword(credentialsId: 'az-jenkins-sp', usernameVariable: 'SERVICE_PRINCIPAL_ID', passwordVariable: 'SERVICE_PRINCIPAL_PASSWORD'),
                    string(credentialsId: 'AZURE_STORAGE_ACCOUNT', variable: 'AZURE_STORAGE_ACCOUNT'),
                    string(credentialsId: 'AZURE_STORAGE_CONTAINER', variable: 'AZURE_STORAGE_CONTAINER'),
                    string(credentialsId: 'AZURE_TENANT_ID', variable: 'AZURE_TENANT_ID')
                ]) {
                    script {
                        sh """
                        az login --service-principal -u ${SERVICE_PRINCIPAL_ID} -p ${SERVICE_PRINCIPAL_PASSWORD} --tenant $AZURE_TENANT_ID

                        # Upload the tarball to Azure Blob Storage
                        az storage blob upload \
                            --container-name ${AZURE_STORAGE_CONTAINER} \
                            --file enarx.tar.gz \
                            --name ${GIT_TAG}.tar.gz \
                            --account-name ${AZURE_STORAGE_ACCOUNT}
                        """
                    }
                }
            }
        }
    }

    post {
        always {
            cleanWs()
        }
    }
}

