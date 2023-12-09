#include </home/etonit/advent-of-code2023/util.c>
#include <limits.h>

/*
destination source range
	source + i => destination + i | i < range
	if not mapped source => destination
 */

int main(int argc, char** argv){
	char* input = readfile("/home/etonit/advent-of-code2023/5day/input");
	LINKEDLIST* sections = splitAtSubstring(input, "\n\n");
	printf("splitted into sections\n");

	LINKEDLIST* seeds = splitStr(sections->data, ' ')->next;

	LINKEDLIST* maps = sections->next;
	printf("created seeds and maps LL\n");


	long source[20];
	long destination[20];
	for(int i = 0; i<20; i++){
		source[i] = longfromstr(seeds->data);
		destination[i] = 0;
		seeds = seeds->next;
	}

	// for every node in maps
	LINKEDLIST* currMap = maps;
	while(currMap){
		printf("working on map:\n%s\n", currMap->data);
		LINKEDLIST* lines = splitStr(currMap->data, '\n')->next;

		// for every source
		for(int srcIdx = 0; srcIdx<20; srcIdx++){
			printf("working with source %li", source[srcIdx]);
			// for every line
			LINKEDLIST* currLine = lines;
			while(currLine){
				printf("working on line:");
				LINKEDLIST* destSrcRang = splitStr(currLine->data, ' ');
				//printList(destSrcRang);

				long dest = longfromstr(destSrcRang->data);
				long src = longfromstr(destSrcRang->next->data);
				long range = longfromstr(destSrcRang->next->next->data);
				printf(" %li %li %li\n", dest, src, range);

				long currSource = source[srcIdx];

				// find if source is in range
				if(currSource < src+range && currSource >= src){
					// if it is map to destination and break
					destination[srcIdx] = dest + (currSource - src);
					break;
				}

				printf("going to next line\n");
				// else go to next line
				currLine = currLine->next;
			}
			// if not found any, destination = source
			if(destination[srcIdx] == 0) {
				destination[srcIdx] = source[srcIdx];
			}
			printf("found destination %li\n", destination[srcIdx]);
		}
		for(int i = 0; i<20; i++){
			source[i] = destination[i];
			destination[i] = 0;
		}


		currMap = currMap->next;
	}

	long smallest = LONG_MAX;
	for(int i = 0; i<20; i++){
		if(source[i] < smallest) { smallest = source[i];}
	}
	printf("The smallest one is %li\n", smallest);


	return 0;
}
