#include <assert.h>
#include <limits.h>
#include <stdbool.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <time.h>

#include <netdb.h>
#include <sys/epoll.h>
#include <unistd.h>

#include <common/idmap.h>
#include <common/stack.h>
#include "clients.h"
#include "net.h"


int sendPosition(int clientFD, size_t id, int xPos, int yPos);


int main(int argc, char const *argv[])
{
	printf("Server started.\n");

	srand((unsigned int)time(NULL));

	net net = net_init("34481");

	const int maxClients = 4;

	stack(size_t) idPool;
	stack_init(idPool, maxClients);

	for (size_t i = maxClients; i > 0; i -= 1)
	{
		stack_push(idPool, i - 1)
	}

	clientMap clients;
	idmap_init(clients, maxClients);

	while (true)
	{
		#define MAX_EVENTS 1024
		struct epoll_event events[MAX_EVENTS];
		int numberOfEvents = epoll_wait(net.pollerFD, events, MAX_EVENTS, 500);
		assert(numberOfEvents != -1);

		for (int i = 0; i < numberOfEvents; i += 1)
		{
			int clientFD = net_acceptClient(net.serverFD);

			if (idPool.size == 0)
			{
				int status = close(clientFD);
				assert(status == 0);
			}
			else
			{
				int xPos = rand() % 600 - 300;
				int yPos = rand() % 400 - 200;

				size_t clientId;
				stack_pop(idPool, &clientId);

				client client = {clientFD, clientId, xPos, yPos};
				idmap_put(clients, clientId, client);
			}
		}

		idmap_each(clients, i,
			idmap_get(clients, i).xPos += 5;
			idmap_get(clients, i).yPos += 0;
		)

		idmap_each(clients, i,
			idmap_each(clients, j,
				int status = sendPosition(
					idmap_get(clients, i).socketFD,
					idmap_get(clients, j).id,
					idmap_get(clients, j).xPos,
					idmap_get(clients, j).yPos);

				if (status < 0)
				{
					idmap_remove(clients, i);
					stack_push(idPool, i);
				}
			)
		)
	}
}

int sendPosition(int clientFD, size_t id, int xPos, int yPos)
{
	char message[256];
	int status = snprintf(
		message + 1, sizeof message - 1,
		"UPDATE id: %lu, pos: (%d, %d)",
		id, xPos, yPos);
	assert(status >= 0);
	assert((size_t)status <= sizeof message);

	size_t messageLength = strlen(message + 1) + 1;
	assert(messageLength <= CHAR_MAX);
	message[0] = (char)messageLength;

	return net_send(clientFD, message, strlen(message));
}
