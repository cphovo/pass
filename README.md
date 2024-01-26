## pass

A simple totp validator demo.

## How To Use

1. Download [Google Authenticator](https://play.google.com/store/apps/details?id=com.google.android.apps.authenticator2)

2. Generate a secret key
   
   ```bash
   > pass generate -l cphovo
   # YOUR SECRET: 7DVDJPXL3XTGGJBRFNI2DLOAMWON34VK
   ```

   then you will see your secret key and a QR Code named qr.png.

3. Scan the QR Code with Google Authenticator

4. Verify the code
   
   ```bash
   > pass verify 7DVDJPXL3XTGGJBRFNI2DLOAMWON34VK -t 250741
   # SUCCESS or INVALID TOKEN
   ```
