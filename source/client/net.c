#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#include <netdb.h>

#include "net.h"

int net_connect(const char *hostname)
{
	int status;

	struct addrinfo hints;
	memset(&hints, 0, sizeof hints);
	hints.ai_family   = AF_UNSPEC;
	hints.ai_socktype = SOCK_STREAM;

	struct addrinfo *servinfo;

	status = getaddrinfo(hostname, "34481", &hints, &servinfo);
	if (status != 0)
	{
		perror("Error getting address info");
		exit(1);
	}

	int socketFD = socket(
		servinfo->ai_family,
		servinfo->ai_socktype,
		servinfo->ai_protocol);
	if (socketFD == -1)
	{
		perror("Error creating socket");
		exit(1);
	}

	status = connect(socketFD, servinfo->ai_addr, servinfo->ai_addrlen);
	if (status != 0)
	{
		perror("Error connecting to server");
		exit(1);
	}

	freeaddrinfo(servinfo);

	return socketFD;
}
