import socket

ip = '127.0.0.1'
# port = 3000
msg = b'Hello, World!'

sock = socket.socket(socket.AF_INET, socket.SOCK_DGRAM)
sock.sendto(msg,(ip, 3000))

ip = '127.0.0.1'
sock = socket.socket(socket.AF_INET, socket.SOCK_DGRAM)
sock.bind((ip, 5000))

data, addr = sock.recvfrom(1024)
print(data.decode())