package main

import (
	"log"
	"net"
	"io"
)

const (
	PROTOCOL = "tcp"
	ADDRESS = "127.0.0.1"
	PORT = "6666"
	BUFSIZ = 128
)

func onClientConnected(connection net.Conn) {
	log.Print("new connection")

	for {
		buffer := make([]byte, BUFSIZ)
		length, err := connection.Read(buffer)

		if err != nil {
			if err != io.EOF {
				log.Print(err.Error())
			}
			break
		}

		if length <= 0 {
			break
		}

		connection.Write(buffer)
	}

	connection.Close()
}

func main() {

	full_address := ADDRESS + ":" + PORT

	listener, err := net.Listen(PROTOCOL, full_address)

	if err != nil {
		log.Fatal(err.Error())
	}

	defer listener.Close()

	log.Print("listening to ", full_address)

	for {
		connection, err := listener.Accept()
		if err != nil {
			log.Fatal(err.Error())
		}

		go onClientConnected(connection)
	}
}