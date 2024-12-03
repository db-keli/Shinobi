# Shinobi

<p align="left">
  <img src="./忍.png" width="150" align="left" style="margin-right: 10px;" />
  A secure client-server tool that allows project owners to manage builds without exposing sensitive information. The server encrypts sensitive keys and generates a token, which is then encoded into a QR code. Clients can scan the QR code kick off a secrets server(daemon) that uses a token from the qrcode to retrieve the keys, the secrets server stores it in a secure memory block which i later safely retrieved by a client library in code. This approach helps ensure secure handling of credentials in collaborative environments.
</p>

<br clear="left" />

##### Design

<p align="center">
  <img src="./shinobi_architecture.png" />
</p>

<a href="https://www.buymeacoffee.com/dompehbright" target="_blank"><img src="https://cdn.buymeacoffee.com/buttons/v2/default-yellow.png" alt="Buy Me A Coffee" style="height: 60px !important;width: 217px !important;" ></a>

work in progress
