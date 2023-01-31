import {
  HandleLogin,
  HandleRegistration,
  Login,
  Registration,
  ServerSetup,
  set_panic_hook,
} from 'opaque-wasm'

const password = 'test123'
const email = 'dave@test.com'

async function run() {
  console.log('--- STARTING ---')
  set_panic_hook()

  // Server configuration; this must be saved.
  let server_setup = new ServerSetup()

  // Save and reload the ServerSetup for demonstration
  const server_setup_export = server_setup.serialize()
  server_setup = ServerSetup.deserialize(server_setup_export)

  // User registration
  const client_registration = new Registration()
  const registration_request = client_registration.start(password)

  console.log('--- registration request ---', registration_request)

  const serverRegistration = new HandleRegistration(server_setup)
  const registration_response = serverRegistration.start(
    email,
    registration_request
  )
  console.log('-- registration response --', registration_response)

  const registration_record = client_registration.finish(
    password,
    registration_response
  )
  console.log('-- registration upload --', registration_record)

  const password_file = serverRegistration.finish(registration_record)
  console.log('-- password_file --', password_file)
  const registration_export_key = client_registration.getExportKey()

  // User Login

  const client_login = new Login()
  const login_request = client_login.start(password)
  console.log('login_request', login_request)

  const server_login1 = new HandleLogin(server_setup)
  const login_response = server_login1.start(
    password_file,
    email,
    login_request
  )
  console.log('login_response', login_response)
  // Serialize login handler state and persist it outside of the server
  // to preserve statlessness across request handlers
  const login_state = server_login1.serialize()
  server_login1.free()
  console.log('login_state', login_state)

  const login_final = client_login.finish(password, login_response)
  console.log('client login final', login_final)
  console.log('client session key', client_login.getSessionKey())

  const server_login2 = HandleLogin.deserialize(login_state, server_setup)
  const server_finish = server_login2.finish(login_final)
  console.log('server session key', server_finish)

  console.log('registration export key', registration_export_key)
  console.log('login export key', client_login.getExportKey())
}

run()
