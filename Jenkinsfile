#!/usr/bin/env groovy

pipeline {
    agent any

    environment {
        APPLICATION_NAME = 'pdf-gen'
        FASIT_ENV = 'q1'
        ZONE = 'fss'
        NAMESPACE = 'default'
        COMMIT_HASH_SHORT = gitVars 'commitHashShort'
        COMMIT_HASH = gitVars 'commitHash'
    }

    stages {
        stage('initialize') {
            steps {
                script {
                    def cargoToml = readFile "${env.WORKSPACE}/Cargo.toml"
                    def matcher = cargoToml =~ 'version *= *"([0-9.]*)"'
                    env.APPLICATION_VERSION = matcher[0][1]
                    echo "Building ${env.APPLICATION_NAME} ${env.APPLICATION_VERSION}"
                    changeLog = utils.gitVars(env.APPLICATION_NAME).changeLog.toString()
                    githubStatus 'pending'
                    slackStatus status: 'started', changeLog: "${changeLog}"
                }
            }
        }

        stage('build') {
            steps {
                dockerUtils 'createPushImage'
            }
        }
        stage('validate & upload nais.yaml to nexus') {
            steps {
                nais 'validate'
                nais 'upload'
            }
        }
        stage('deploy to preprod') {
            steps {
                deployApplication()
            }
        }

    }
    post {
        always {
            ciSkip 'postProcess'
            dockerUtils 'pruneBuilds'
            script {
                if (currentBuild.result == 'ABORTED') {
                    slackStatus status: 'aborted'
                }
            }
            junit '**/build/test-results/test/*.xml'
            archiveArtifacts artifacts: 'build/reports/rules.csv', allowEmptyArchive: true
            archiveArtifacts artifacts: '**/build/libs/*', allowEmptyArchive: true
            archiveArtifacts artifacts: '**/build/install/*', allowEmptyArchive: true
            deleteDir()
        }
        success {
            githubStatus 'success'
            slackStatus status: 'success'
        }
        failure {
            githubStatus 'failure'
            slackStatus status: 'failure'
        }
    }
}

void deployApplication() {
    def jiraIssueId = nais 'jiraDeploy'
    slackStatus status: 'deploying', jiraIssueId: "${jiraIssueId}"
    try {
        timeout(time: 1, unit: 'HOURS') {
            input id: "deploy", message: "Waiting for remote Jenkins server to deploy the application..."
        }
    } catch (Exception exception) {
        currentBuild.description = "Deploy failed, see " + currentBuild.description
        throw exception
    }
}
