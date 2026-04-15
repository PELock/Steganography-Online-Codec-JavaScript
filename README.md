# Steganography Online Codec SDK for JavaScript & NPM

**Steganographic Online Codec** allows you to hide a password encrypted message within the images & photos using [AES](https://www.youtube.com/watch?v=O4xNJsjtN6E)
encryption algorithm with a 256-bit [PBKDF2](https://en.wikipedia.org/wiki/PBKDF2) derived key.

You can use it for free at:

https://www.pelock.com/products/steganography-online-codec

This SDK provides programming access to the codec and its encoding and decoding functions through a WebAPI interface.

## What is steganography & how it works?

Steganography is a term describing the art and science of hiding information by embedding messages within other, seemingly harmless image files.

In this case, the individual bits of the encrypted hidden message are saved as the least significant (LSB) bits in the
RGB color components in the pixels of the selected image.

![Steganography Online Codec - Hide Message in Image](https://www.pelock.com/img/en/products/steganography-online-codec/steganography-online-codec.png)

With our steganographic encoder you will be able to conceal any text message in the image in a secure way and
send it without raising any suspicion. It will only be possible to read the message after providing valid, decryption
password.

## Installation

The preferred way of Web API SDK installation is via [NPM](https://www.npmjs.com/) (Node Package Manager).

Run:

```
npm i steganography-online-codec
```

Or add this entry:

```
  "dependencies": {
    "steganography-online-codec": "latest"
  },

```

directly to your `package.json` in `dependencies` section.

The installation package is available at https://www.npmjs.com/package/steganography-online-codec

## Packages for other programming languages

The installation packages have been uploaded to repositories for several popular programming languages and their source codes have been published on GitHub:

| Repository   | Language | Installation | Package | GitHub |
| ---------- | -------- | ------------ | ------- | ------ |
| ![PyPI repository for Python](https://www.pelock.com/img/logos/repo-pypi.png) | Python | Run `pip install steganography-online-codec` | [PyPI](https://pypi.org/project/steganography-online-codec/) | [Sources](https://github.com/PELock/Steganography-Online-Codec-Python) |
| ![NPM repository for JavaScript and TypeScript](https://www.pelock.com/img/logos/repo-npm.png) | JavaScript, TypeScript | Run `npm i steganography-online-codec` or add `"steganography-online-codec": "latest"` under `dependencies` in `package.json` | [NPM](https://www.npmjs.com/package/steganography-online-codec) | [Sources](https://github.com/PELock/Steganography-Online-Codec-JavaScript) |
| ![Rust crates.io](https://www.pelock.com/img/logos/repo-crates.png) | Rust | Run `cargo add steganography-online-codec` or add `steganography-online-codec = "1"` to `Cargo.toml` (optional: `git` / `path` for unreleased or vendored builds) | [crates.io](https://crates.io/crates/steganography-online-codec) | [Sources](https://github.com/PELock/Steganography-Online-Codec-Rust) |

#### Alternative usage

If you don't want to use Python module, you can import directly from the file:

```python
from pelock.steganography_online_codec import *
```

### How to hide a secret message within an image file

```js
"use strict";

/******************************************************************************
 *
 * Steganography Online Codec WebApi interface usage example.
 *
 * In this example shows how to hide an encrypted secret message in an image file.
 *
 * Version      : v1.00
 * Language     : JavaScript
 * Author       : Bartosz Wójcik (original example)
 * Project      : https://www.pelock.com/products/steganography-online-codec
 * Homepage     : https://www.pelock.com
 *
 * @link https://www.pelock.com/products/steganography-online-codec
 * @copyright Copyright (c) 2020-2025 PELock LLC
 * @license Apache-2.0
 *
/*****************************************************************************/

// include Steganography Online Codec module
import { SteganographyOnlineCodec, Errors } from 'steganography-online-codec';
// or if tested locally use:
//import { SteganographyOnlineCodec, Errors } from '../src/SteganographyOnlineCodec.mjs';

// create Steganography Online Codec class instance (we are using our activation key)
const mySteganographyOnlineCodec = new SteganographyOnlineCodec('YOUR-WEB-API-KEY');

// encode a hidden message (encrypted with your password) within an image file
(async () => {

	const inputFile = 'input_file.jpg';
	const secretMessage = 'Secret message';
	const password = 'Pa$$word';
	const outputFile = 'output_file_with_hidden_secret_message.png';

	try {
		const result = await mySteganographyOnlineCodec.encode(inputFile, secretMessage, password, outputFile);

		// result object holds the encoding results as well as other information
		console.log('Secret messaged encoded and saved to the output PNG file.');
	} catch (err) {
		console.error('Encoding failed:', err.error_message || err.message || String(err));
	}
})();
```

### More complex example with better explanation and proper error codes checking

```js
"use strict";

/******************************************************************************
 *
 * Steganography Online Codec WebApi interface usage example.
 *
 * In this example, we will see how to hide an encrypted message in an
 * image file using our codec.
 *
 * Version      : v1.00
 * Language     : JavaScript
 * Author       : Bartosz Wójcik
 * Project      : https://www.pelock.com/products/steganography-online-codec
 * Homepage     : https://www.pelock.com
 *
 * @link https://www.pelock.com/products/steganography-online-codec
 * @copyright Copyright (c) 2020-2025 PELock LLC
 * @license Apache-2.0
 *
 /*****************************************************************************/

// include Steganography Online Codec module
import { SteganographyOnlineCodec, Errors } from 'steganography-online-codec';
// or if tested locally use:
//import { SteganographyOnlineCodec, Errors } from '../src/SteganographyOnlineCodec.mjs';


// create Steganography Online Codec class instance (we are using our activation key)
const mySteganographyOnlineCodec = new SteganographyOnlineCodec('YOUR-WEB-API-KEY');

// encode a hidden message within the source image file
(async () => {
	// full version image size limit is set to 10 MB (demo 50 kB max)
	// supported image formats are PNG, JPG, GIF, BMP, WBMP, GD2, AVIF, WEBP (mail me for more)
	const inputFilePath = 'input_file.webp';

	// full version message size is unlimited (demo 16 chars max)
	const secretMessage = 'Secret message';

	// full version password length is 128 characters max (demo 8 chars max)
	const password = 'Pa$$word';

	// where to save encoded image with the secret message
	const outputFilePath = 'output_file_with_hidden_secret_message.png';

	try {
		// encode a hidden message (encrypted with your password) within an image file
		const result = await mySteganographyOnlineCodec.encode(inputFilePath, secretMessage, password, outputFilePath);

		// result object holds the encoding results as well as other information
		const versionType = result.license && result.license.activationStatus ? 'full' : 'demo';
		console.log(`You are running in ${versionType} version`);

		console.log(`Secret messaged encoded and saved to ${outputFilePath}`);
		if (result.license && result.license.usagesCount !== undefined) {
			console.log(`Remaining number of usage credits - ${result.license.usagesCount}`);
		}
	} catch (err) {
		const errorCode = err.error;

		switch (errorCode) {
			case Errors.INVALID_INPUT:
				console.log(`Invalid input file ${inputFilePath} or file doesn't exist`);
				break;
			case Errors.MESSAGE_TOO_LONG:
				console.log('Message is too long for the provided image file, use larger file');
				break;
			case Errors.IMAGE_TOO_BIG:
				console.log(`Image file is too big, current limit is set to ${err.raw?.limits?.maxFileSize ?? 'unknown'}`);
				break;
			case Errors.LIMIT_MESSAGE:
				console.log(`Message is too long, current limit is set to ${err.raw?.limits?.maxMessageLen ?? 'unknown'}`);
				break;
			case Errors.LIMIT_PASSWORD:
				console.log(`Password is too long, current limit is set to ${err.raw?.limits?.maxPasswordLen ?? 'unknown'}`);
				break;
			case Errors.INVALID_PASSWORD:
				console.log('Invalid password');
				break;
			default:
				console.log(`An error occurred: ${err.error_message ?? `Error code ${errorCode}`}`);
		}
	}
})();
```

### How to extract encoded secret message from the image file

```js
"use strict";

/******************************************************************************
 *
 * Steganography Online Codec WebApi interface usage example.
 *
 * In this example, we will see how to extract a previously encrypted & hidden
 * secret message from an image file.
 *
 * Version      : v1.00
 * Language     : JavaScript
 * Author       : Bartosz Wójcik
 * Project      : https://www.pelock.com/products/steganography-online-codec
 * Homepage     : https://www.pelock.com
 *
 * @link https://www.pelock.com/products/steganography-online-codec
 * @copyright Copyright (c) 2020-2025 PELock LLC
 * @license Apache-2.0
 *
 /*****************************************************************************/

// include Steganography Online Codec module
import { SteganographyOnlineCodec, Errors } from 'steganography-online-codec';
// or if tested locally use:
//import { SteganographyOnlineCodec, Errors } from '../src/SteganographyOnlineCodec.mjs';

// create Steganography Online Codec class instance (we are using our activation key)
const mySteganographyOnlineCodec = new SteganographyOnlineCodec('YOUR-WEB-API-KEY');

// extract a hidden message from the previously encoded image file
(async () => {
	// full version image size limit is set to 10 MB (demo 50 kB max)
	// supported image format is PNG and only PNG!
	const inputFilePath = 'output_file_with_hidden_secret_message.png';

	// full version password length is 128 characters max (demo 8 chars max)
	const password = 'Pa$$word';

	try {
		// extract a hidden message from the image (PNG files only)
		const result = await mySteganographyOnlineCodec.decode(inputFilePath, password);

		// result object holds the decoding results as well as other information
		console.log(`You are running in ${result.license?.activationStatus ? 'full' : 'demo'} version`);

		console.log(`Secret message is "${result.message}"`);

		if (result.license && result.license.usagesCount !== undefined) {
			console.log(`Remaining number of usage credits - ${result.license.usagesCount}`);
		}
	} catch (err) {
		switch (err.error) {
			case Errors.INVALID_INPUT:
				console.log(`Invalid input file ${inputFilePath} or file doesn't exist`);
				break;
			case Errors.IMAGE_TOO_BIG:
				console.log(`Image file is too big, current limit is set to ${err.raw?.limits?.maxFileSize ?? 'unknown'}`);
				break;
			case Errors.LIMIT_MESSAGE:
				console.log(`Extracted message is too long, current limit is set to ${err.raw?.limits?.maxMessageLen ?? 'unknown'}`);
				break;
			case Errors.LIMIT_PASSWORD:
				console.log(`Password is too long, current limit is set to ${err.raw?.limits?.maxPasswordLen ?? 'unknown'}`);
				break;
			case Errors.INVALID_PASSWORD:
				console.log('Invalid password');
				break;
			default:
				console.log(`An error occurred: ${err.error_message ?? String(err)}`);
		}
	}
})();
```

### How to check the license key status & current limits

```js
"use strict";

/******************************************************************************
 *
 * Steganography Online Codec WebApi interface usage example.
 *
 * In this example we will verify our activation key status.
 *
 * Version      : v1.00
 * Language     : JavaScript
 * Author       : Bartosz Wójcik
 * Project      : https://www.pelock.com/products/steganography-online-codec
 * Homepage     : https://www.pelock.com
 *
 * @link https://www.pelock.com/products/steganography-online-codec
 * @copyright Copyright (c) 2020-2025 PELock LLC
 * @license Apache-2.0
 *
 /*****************************************************************************/

// include Steganography Online Codec module
import { SteganographyOnlineCodec, Errors } from 'steganography-online-codec';
// or if tested locally use:
//import { SteganographyOnlineCodec, Errors } from '../src/SteganographyOnlineCodec.mjs';

// create Steganography Online Codec class instance (we are using our activation key)
const mySteganographyOnlineCodec = new SteganographyOnlineCodec('YOUR-WEB-API-KEY');

// login to the service
(async () => {
	try {
		const result = await mySteganographyOnlineCodec.login();

		// result object holds the information about the license & current limits
		const versionType = result.license && result.license.activationStatus ? 'full' : 'demo';
		console.log(`You are running in ${versionType} version`);

		// information about the current license
		if (result.license && result.license.activationStatus) {
			console.log(`Registered for - ${result.license.userName}`);
			const licenseType = result.license.type === 0 ? 'personal' : 'company';
			console.log(`License type - ${licenseType}`);
			console.log(`Total number of purchased usage credits - ${result.license.usagesTotal}`);
			console.log(`Remaining number of usage credits - ${result.license.usagesCount}`);
		}

		// current limits (different for DEMO and FULL versions)
		if (result.limits) {
			console.log(`Max. password length - ${result.limits.maxPasswordLen}`);
			const msgLen = result.limits.maxMessageLen === -1 ? 'unlimited' : result.limits.maxMessageLen;
			console.log(`Max. message length - ${msgLen}`);
			console.log(`Max. input image file size - ${mySteganographyOnlineCodec.convert_size(result.limits.maxFileSize)}`);
		}
	} catch (err) {
		console.error(`Login failed: ${err.error_message || String(err)}`);
	}
})();
```

## Got questions?

If you are interested in the Steganography Online Codec Web API or have any questions regarding SDK packages, technical or if something is not clear, [please contact me](https://www.pelock.com/contact). I'll be happy to answer all of your questions.

Bartosz Wójcik

* Visit my site at — https://www.pelock.com
* X — https://x.com/PELock
* GitHub — https://github.com/PELock