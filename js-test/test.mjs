import { Registration, Login, HandleRegistration, HandleLogin, ServerSetup } from 'opaque-wasm';

const password = 'test123'
const email = 'dave@test.com'

async function run() {
  console.log('--- STARTING ---')

  // Server configuration; this must be saved.
  let server_setup = new ServerSetup()

  // Save and reload the ServerSetup for demonstration
  const server_setup_export = server_setup.serialize();
  server_setup = ServerSetup.deserialize(server_setup_export);
  
  // User registration
  const registration = new Registration()
  const registration_tx = registration.start(password)

  console.log('--- begin ---', registration_tx)

  const serverRegistration = new HandleRegistration(server_setup)
  const registration_response = serverRegistration.start(email, registration_tx)
  console.log('-- server response --', registration_response)


  const registration_final = registration.finish(registration_response)
  console.log('-- client finish --', registration_final)

  const password_file = serverRegistration.finish(registration_final)
  console.log('-- password_file --', password_file)

  // User Login

  const login = new Login()
  const login_tx = login.start(password)
  console.log('login_tx', login_tx)

  console.log(login)

  const serverLogin = new HandleLogin(server_setup)
  const login_response = serverLogin.start(password_file, email, login_tx)

  console.log('login_response', login_response)

  const login_final = login.finish(login_response)
  console.log('client login final', login_final)

  console.log(login)

  console.log('client session key', login.getSessionKey())

  const server_finish = serverLogin.finish(login_final)
  console.log('server session key', server_finish)
}

run()