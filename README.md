# Squirrel

![Rust](https://img.shields.io/badge/rust-%23000000.svg?style=for-the-badge&logo=rust&logoColor=white)
![YAML](https://img.shields.io/badge/yaml-%23ffffff.svg?style=for-the-badge&logo=yaml&logoColor=151515)
![Selenium](https://img.shields.io/badge/-selenium-%43B02A?style=for-the-badge&logo=selenium&logoColor=white)

![release](https://img.shields.io/github/v/release/goyalmunish/squirrel)
[![Crates.io](https://img.shields.io/crates/v/shield-maker.svg)](https://crates.io/crates/shield-maker)
![build](https://img.shields.io/github/actions/workflow/status/goyalmunish/squirrel/ci.yaml?label=build)
![license](https://img.shields.io/github/license/goyalmunish/squirrel)

```ascii
    ░░░░░░░░░██▄▒█▀▄░▄▀▄░█░░▒█░▄▀▀▒██▀▒█▀▄░░▒▄▀▄░█▒█░▀█▀░▄▀▄░█▄▒▄█▒▄▀▄░▀█▀░█░▄▀▄░█▄░█░░░░░░░░
    ▒░▒░▒░▒░▒█▄█░█▀▄░▀▄▀░▀▄▀▄▀▒▄██░█▄▄░█▀▄▒░░█▀█░▀▄█░▒█▒░▀▄▀░█▒▀▒█░█▀█░▒█▒░█░▀▄▀░█▒▀█▒░▒░▒░▒░
    
                   .^!777!^.                                                        
                 ^5#@@@@@@@&GJ^                               !5^.                  
                ?@@&#&&@@@@@@@@G!                           .J@@&PGJ:               
               .7^:...:~75B@@@@@@G!                  ..:^~?P&@@@@B@@@J              
                           ~5&@@@@@G~         :~?YPGB#&&@@@@@@@@&GPY7.              
                             :?G@@@@@G7.   ^YB&@@@@@@@@@@@@@@@@@G.                  
                                ^?PB&@@BY~Y@@@@@@@@@@@@@@@@@@@@@@&GY?7~:            
                                    .::..P@@@@@@&B5J7~~^^^^^^~!77??7!^.             
                                        5@@@@@#7:                                   
                                       Y@@@@B?.                                     
                                     :G@&GJ~                                        
                                    .#G!.                                           
                                    :5                                              
                                     .
    
    ░░░░░░░░░░░░░░░░░░░░░█░░▒█░█░▀█▀░█▄█░░░░░▄▀▀░▄▀▄░█▒█░█▒█▀▄▒█▀▄▒██▀░█▒░░░░░░░░░░░░░░░░░░░░░
    ▒░▒░▒░▒░▒░▒░▒░▒░▒░▒░░▀▄▀▄▀░█░▒█▒▒█▒█▒░▒░▒▄██░▀▄█░▀▄█░█░█▀▄░█▀▄░█▄▄▒█▄▄▒░▒░▒░▒░▒░▒░▒░▒░▒░▒░
```

Squirrel revolutionizes browser automation by simplifying the process through YAML based workflow (such as [this sample workflow](./src/sample_workflow.yaml)) definition. With Squirrel, you effortlessly automate tasks without getting bogged down by technical intricacies, as the tool handles all the underlying complexities for you.

- [Github Repo](https://github.com/goyalmunish/squirrel)
- [Cargo Package: `squirrel-browser-automation`](https://crates.io/crates/squirrel-browser-automation)
- [Documentation](https://docs.rs/crate/squirrel-browser-automation/latest)
- [**Supported Operations**](https://github.com/goyalmunish/squirrel/blob/main/src/wf/workflow_step.rs)

Here is a glimpse of the [above workflow in action](./assets/screen_recording_squirrel_sample_workflow_20240101.mp4)

[![Sample Workflow in Action](./assets/screen_recording_squirrel_sample_workflow_20240101_thumbnail.png)](./assets/screen_recording_squirrel_sample_workflow_20240101.mp4)

## Quick Start

### Run Squirrel

_**Option 1:** Using both Squirrel executable and Web Driver from Docker images (experimental)_

_Refer [Install Docker Engine](https://docs.docker.com/engine/install/) for Docker installation. This option runs the chrome driver via a Docker container, so it doesn't need you to setup chrome driver locally. One of the benefit of using container based chrome driver is that you are not imposed with required support for the default window size `(1293 x 1200)` used by Squirrel._

_For docker container to host machine connectivity issues, refer [I want to connect from a container to a service on the host](https://docs.docker.com/desktop/networking/#i-want-to-connect-from-a-container-to-a-service-on-the-host)._

_Refer:_

- [`SeleniumHQ/docker-selenium`](https://github.com/SeleniumHQ/docker-selenium) (Selenium Docker Images)
- [`seleniumhq-community/docker-seleniarm`](https://github.com/seleniumhq-community/docker-seleniarm) (Selenium Docker Images for Arm64)

```shell
# Run Selenium Chromium Driver on MacOS (Arm64)
docker run --rm -it -p 4444:4444 -p 7900:7900 --shm-size="2g" -v /dev/shm:/dev/shm --name chrome seleniarm/standalone-chromium:latest

# Create a custom workflow file
mkdir temp
curl -o workflow.yaml https://raw.githubusercontent.com/goyalmunish/squirrel/main/src/sample_workflow.yaml
cp workflow.yaml temp/workflow.yaml # you may update the workflow, with existing steps as example

# Run Squirrel
docker run --rm -v $(pwd)/temp:/app/temp --name squirrel goyalmunish/squirrel /app/temp/workflow.yaml http://host.docker.internal:4444 true "--no-sandbox --disable-dev-shm-usage --disable-popup-blocking --disable-gpu"
```

_In the above example, the execution was conducted in browser using headless mode. However, if you prefer to proceed in non-headless mode, simply set the `headless_browser` to `false`. This will allow you to connect to the remove bowser via http://localhost:7900/?autoconnect=1&resize=scale&password=secret. Refer [Using your Browser](https://github.com/seleniumhq-community/docker-seleniarm?#using-your-browser-no-vnc-client-is-needed) for more details._

**Option 2:** Using Squirrel executable from the Docker image (preferred)

Refer [Install Docker Engine](https://docs.docker.com/engine/install/) for Docker installation, and [Setup Chrome for Testing](#setup-chrome-for-testing) for local setup of Chrome Driver.

For docker container to host machine connectivity issues, refer [I want to connect from a container to a service on the host](https://docs.docker.com/desktop/networking/#i-want-to-connect-from-a-container-to-a-service-on-the-host).

```shell
# Run with sample workflow (with default option of webdriver_url=http://host.docker.internal:9515 and headless_browser=true)
docker run --rm --name squirrel goyalmunish/squirrel

# or, run specified workflow without headless browser
docker run --rm --name squirrel goyalmunish/squirrel ./src/sample_workflow.yaml http://host.docker.internal:9515 false

# Here, is an example of running in headless mode with custom workflow & configurations, and shared workflow & output directory
mkdir temp
curl -o workflow.yaml https://raw.githubusercontent.com/goyalmunish/squirrel/main/src/sample_workflow.yaml
cp workflow.yaml temp/workflow.yaml # update the workflow, with existing steps as example
docker run --rm -v $(pwd)/temp:/app/temp --name squirrel goyalmunish/squirrel /app/temp/workflow.yaml http://host.docker.internal:9515 true "--no-sandbox --disable-dev-shm-usage --disable-popup-blocking --disable-gpu"
```

**Option 3:** Using the locally build Squirrel executable

Refer [Setup Chrome for Testing](#setup-chrome-for-testing) for local setup of Chrome Driver.

```shell
# Clone the repo
git clone git@github.com:goyalmunish/squirrel.git
# cd into projet directory
cd squirrel
# Run with specified workflow (with default option of webdriver_url=http://localhost:9515 and headless_browser=true)
cp ./src/sample_workflow.yaml workflow.yaml     # update the workflow, with existing steps as example
cargo run --release workflow.yaml
```

### Setup Chrome for Testing

Follow these steps to set up Chrome for testing:

Refer [Chrome for Testing: reliable downloads for browser automation](https://developer.chrome.com/blog/chrome-for-testing/) and [Chrome for Testing availability](https://googlechromelabs.github.io/chrome-for-testing/). But, here is the gist:

Download the latest stable Chrome for Testing binary and ChromeDriver:

```shell
# Download the latest available Chrome for Testing binary corresponding to the Stable channel.
npx @puppeteer/browsers install chrome@stable

# Download the latest available ChromeDriver version corresponding to the Stable channel.
npx @puppeteer/browsers install chromedriver@stable
```

Make them executable:

```shell
# Setup executables
cd ~
mv chrome chromeDIR
mv chromedriver chromedriverDIR
ln -s chromeDIR/.../Google\ Chrome\ for\ Testing.app Google\ Chrome\ for\ Testing.app
# note: for macOS, drag a copy of Google Chrome for Testing app to "Applications" as well
ln -s chromedriverDIR/.../chromedriver chromedriver
```

Invoke Chrome Driver:

```shell
# Run webdriver
cd ~
./chromedriver --verbose --port=9515 --allowed-origins='*' --allowed-ips='0.0.0.0'
```

Once the webdriver is running, you may test by connecting to http://0.0.0.0:9515/.

## Development Guide

### Build and Push the Docker Image

_Make use of [`build_image.sh`](./scripts/build_image.sh) to build and push (requires admin rights) [Docker image](https://hub.docker.com/r/goyalmunish/squirrel/tags):_

```sh
# cd into repo
cd squirrel/

# set version, for example
VERSION=v1.0.0

# build images and pushing them
. ./scripts/build_image.sh ${VERSION}
```
