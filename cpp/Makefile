CC=g++
SOURCES=*.cpp
OBJECTS=$(SOURCES:.cpp=.o)
EXECUTABLE=hello

hello: $(SOURCES)
	$(CC) -Wall $(SOURCES) -o hello

armonly:
	vagrant ssh -c "arm-linux-gnueabihf-g++ -Wall -Wextra -Werror -lstdc++ /vagrant/cpp/*.cpp -o /vagrant/cpp/armhello"	

arm: armonly deploy

deploy:
	scp armhello pi@192.168.1.112:~

