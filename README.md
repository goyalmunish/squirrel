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
ln -s chromeDIR/.../Google\ Chrome\ for\ Testing.app Google\ Chrome\ for\ Testing.app
# note: for macOS, drag a copy of Google Chrome for Testing app to "Applications" as well
ln -s chromedriverDIR/.../chromedriver chromedriver

# Run webdriver
cd ~
./chromedriver --verbose --port=9515 --allowed-origins='*' --allowed-ips='0.0.0.0'
```

Once the webdriver is running, you may test by connecting to http://0.0.0.0:9515/.

### Step 2: Run Squirrel

Option 1: Using Squirrel executable from published Docker image (preferred)

For docker container to host machine connectivity issues, refer [I want to connect from a container to a service on the host](https://docs.docker.com/desktop/networking/#i-want-to-connect-from-a-container-to-a-service-on-the-host).

```shell
# Run with sample workflow (with default option of webdriver_url=http://host.docker.internal:9515 and headless_browser=true)
docker run --rm --name squirrel goyalmunish/squirrel

# Run with your own workflow without headless browser
docker run --rm --name squirrel goyalmunish/squirrel ./src/sample_workflow.yaml http://host.docker.internal:9515 false
```

Option 2: Using the Squirrel executable build locally

```shell
# Clone the repo
git clone git@github.com:goyalmunish/squirrel.git

cd squirrel

# Run with sample workflow (with default option of webdriver_url=http://localhost:9515 and headless_browser=true)
cargo run --release ./src/sample_workflow.yaml
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