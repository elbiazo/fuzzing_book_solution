#include <stdio.h>
#include <string.h>

int main(int argc, char **argv) {
	char weekday[9];
	strcpy(weekday, argv[1]);	
	printf("%s\n", weekday);

	return 0;
}
