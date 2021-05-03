import { Registration, Login, HandleRegistration, HandleLogin } from 'opaque-wasm';

const password = 'test123'
const email = 'dave@test.com'

const server_privatekey = 'c95843d9c67f7ba7f1231af10e1a88dc' // XXX: must be 32 chars long

async function run() {
    console.log('--- STARTING ---')

    // User registration
    const registration = new Registration()
    const registration_tx = registration.start(password)

    console.log('--- begin ---',registration_tx)

    const serverRegistration = new HandleRegistration()
    const registration_response = serverRegistration.start(registration_tx, server_privatekey)
    console.log('-- server response --', registration_response)


    const registration_final = registration.finish(registration_response)
    console.log('-- client finish --',registration_final)

    const password_file = serverRegistration.finish(registration_final)
    console.log('-- password_file --', password_file)


    // User Login

    const login = new Login()
    const login_tx = login.start(password)
    console.log('login_tx', login_tx)

    console.log(login)

    const serverLogin = new HandleLogin()
    const login_response = serverLogin.start(password_file, login_tx, server_privatekey)
    console.log('login_response',login_response)

    const login_final = login.finish(login_response)
    console.log('client login final', login_final)

    console.log(login)

    console.log('client session key',login.getSessionKey())

    const server_finish = serverLogin.finish(login_final)
    console.log('server session key', server_finish)

}

run()