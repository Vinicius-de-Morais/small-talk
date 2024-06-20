const net = require('net');
const readline = require('readline');

const messageHistory = [];

const rl = readline.createInterface({
    input: process.stdin,
    output: process.stdout
});

function clearScreen() {
    process.stdout.write('\x1B[2J\x1B[0f');
}

function printMessages() {
    clearScreen();
    messageHistory.forEach(msg => {
        console.log(msg);
    });
}

function parseResponse(response) {
    const parts = response.split('\r\n');
    if (parts.length >= 3 && parts[0] === "SEND") {
        const requestInfo = JSON.parse(parts[1]);
        const payload = JSON.parse(parts[2].replace("Payload: ", ""));
        return { requestInfo, payload };
    } else {
        throw new Error("Formato de resposta desconhecido");
    }
}

function receiveResponse(socket) {
    socket.on('data', data => {
        try {
            const response = data.toString();
            if (response) {
                const { requestInfo, payload } = parseResponse(response);
                messageHistory.push(`Mensagem recebida: ${payload.message}`);
                printMessages();
            }
        } catch (error) {
            console.error(`Erro ao receber mensagem: ${error.message}`);
        }
    });

    socket.on('error', error => {
        console.error(`Erro no socket: ${error.message}`);
    });

    socket.on('close', () => {
        console.log('Conexão fechada pelo servidor.');
        process.exit(0);
    });
}

function sendRequest(socket, userIp, message) {
    const request = "SEND\r\n" +
        JSON.stringify({
            request_id: 11876854719037224982,
            status: "Okay",
            success: true,
            user: 123,
            user_name: userIp,
            channel: "/"
        }) + "\r\n" +
        'Payload: ' + JSON.stringify({
            command: "/message",
            input: message
        }) + "\r\n";
    socket.write(request);
}

function connectToServer(host, port) {
    return new Promise((resolve, reject) => {
        const client = new net.Socket();
        client.connect(port, host, () => {
            resolve(client);
        });
        client.on('error', (err) => {
            reject(err);
        });
    });
}

async function main() {
    const serverIp = await new Promise((resolve) => {
        rl.question("Digite o IP do servidor: ", resolve);
    });
    const userIp = await new Promise((resolve) => {
        rl.question("Digite seu IP: ", resolve);
    });

    const host = serverIp;
    const port = 6969;

    try {
        const socketConnection = await connectToServer(host, port);
        receiveResponse(socketConnection);

        console.log("Digite 'SAIR' para sair do chat.");

        rl.on('line', (input) => {
            if (input.toLowerCase() === 'sair') {
                socketConnection.end();
                rl.close();
            } else {
                messageHistory.push(`Você: ${input}`);
                sendRequest(socketConnection, userIp, input);
                printMessages();
            }
        });
    } catch (error) {
        console.error(`Erro ao conectar ao servidor: ${error.message}`);
    }
}

main();
