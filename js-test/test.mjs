import { Registration, Login, HandleRegistration, HandleLogin } from 'opaque-wasm';

const password = 'test123'
const email = 'dave@test.com'

const server_setup = [147, 247, 74, 209, 29, 21, 95, 90, 97, 242, 125, 34, 136, 223, 173, 116, 63,
  164, 126, 252, 198, 243, 135, 64, 69, 0, 64, 178, 190, 1, 40, 125, 217, 54, 67,
  187, 66, 173, 121, 60, 150, 7, 101, 19, 74, 224, 228, 130, 168, 5, 96, 88, 127,
  241, 191, 230, 155, 214, 122, 60, 226, 196, 172, 177, 121, 119, 153, 115, 32,
  236, 37, 154, 207, 132, 180, 188, 199, 132, 163, 79, 70, 173, 17, 198, 252,
  112, 86, 159, 28, 97, 140, 96, 30, 209, 42, 9, 137, 44, 124, 226, 127, 150,
  127, 209, 152, 53, 133, 71, 143, 251, 132, 142, 3, 56, 166, 184, 7, 48, 104,
  117, 200, 230, 107, 179, 37, 234, 144, 1]


async function run() {
  console.log('--- STARTING ---')

  // User registration
  const registration = new Registration()
  const registration_tx = registration.start(password)

  console.log('--- begin ---', registration_tx)

  const serverRegistration = new HandleRegistration()
  const registration_response = serverRegistration.start(email, registration_tx, server_setup)
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

  const serverLogin = new HandleLogin()
  const login_response = serverLogin.start(password_file, email, login_tx, server_setup)
  console.log('login_response', login_response)

  const login_final = login.finish(login_response)
  console.log('client login final', login_final)

  console.log(login)

  console.log('client session key', login.getSessionKey())

  const server_finish = serverLogin.finish(login_final)
  console.log('server session key', server_finish)
}

run()