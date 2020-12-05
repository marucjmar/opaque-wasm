<h1><code>opaque-wasm-js</code></h1>

<strong>An implementation of the OPAQUE key exchange protocol in WASM(WebAssembly) hosted by the browser. The library was founded on the basis of [opaque-ke](https://github.com/novifinancial/opaque-ke). </strong>

Note: This package will be published in npm when version 0.3.x of [opaque-ke](https://github.com/novifinancial/opaque-ke) has been stable.

### Build ES6 package

```
wasm-pack build
```

### JS simple example of usage

```js
import { Registration, Login } from "opaque-wasm";

const password = "asdf123";
const email = "myawesomeapp@seerv.dev";

try {
  const registration = new Registration();
  const firstMessage = registration.start(password);
  const secondMessage = await sendMessageToServer(firstMessage);
  const thirdMessage = registration.finish(secondMessage);
  const { status } = await sendMessageToServer(thirdMessage, { email });

  console.log(status); // 204 - Server Return ok, user account has been created

  const login = new Login();
  const firstLoginMessage = login.start(password);
  const secondLoginMessage = await sendMessageToServer(firstLoginMessage, email);
  const thirdLoginMessage = login.finish(secondLoginMessage);
  const { accessToken } = await sendMessageToServer(thirdLoginMessage);

  console.log(accessToken); // eyhojo55....
} catch (e) {
  console.error(e);
}
```
