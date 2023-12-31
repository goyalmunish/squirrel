# Squirrel 🐿️

Squirrel revolutionizes browser automation by simplifying the process through YAML based workflow definition. With Squirrel, you effortlessly automate tasks without getting bogged down by technical intricacies, as the library handles all the underlying complexities for you.

Note: Squirrel is a working product, but it is yet to be polished. Stay tuned for updates and improvements!

## Quick Start

### Step 1: Setup Chrome for Testing

Refer [Chrome for Testing: reliable downloads for browser automation](https://developer.chrome.com/blog/chrome-for-testing/) and [Chrome for Testing availability](https://googlechromelabs.github.io/chrome-for-testing/). But, here is the gist:

```shell
# Download the latest available Chrome for Testing binary corresponding to the Stable channel.
npx @puppeteer/browsers install chrome@stable

# Download the latest available ChromeDriver version corresponding to the Stable channel.
npx @puppeteer/browsers install chromedriver@stable

# Setup executables
cd ~
mv chrome chromeDIR
mv chromedriver chromedriverDIR
ln -s chromeDIR/<>/chrome-mac-arm64/Google\ Chrome\ for\ Testing.app Google\ Chrome\ for\ Testing.app
ln -s chromedriverDIR/<>/chromedriver-mac-arm64/chromedriver chromedriver

# Run webdriver
cd ~
./chromedriver --verbose --port=9515 --allowed-origins=0.0.0.0 --allowed-ips=0.0.0.0
```

### Step 2: Run Squirrel

```shell
# Run with sample workflow (without headless browser)
cargo run --release ./src/sample_workflow.yaml false
```

## Development Guid

## Build and Push the Docker Image

_Make use of [`build_image.sh`](./scripts/build_image.sh) to build and push (requires admin rights) [Docker image](https://hub.docker.com/r/goyalmunish/squirrel/tags):_

```sh
# cd into repo
cd squirrel/

# example, setting version
VERSION=v1.0.0

# building images and pushing them
. ./scripts/build_image.sh ${VERSION}
```