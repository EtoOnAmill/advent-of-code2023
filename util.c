#include <stdio.h>
#include <stdlib.h>
#include <string.h>


#define LINKEDLIST struct LinkedList
struct LinkedList {
	char* data;
	LINKEDLIST* next;
};


int listSize(LINKEDLIST* list){
	LINKEDLIST* next = list->next;
	int size = 1;
	while(next){
		size++;
		next = next->next;
	}
	return size;
}

LINKEDLIST* newList(char* data, size_t dataSize){
	LINKEDLIST* ret = (LINKEDLIST*)malloc(sizeof(LINKEDLIST));

	char* dataMemory = malloc(sizeof(char) * dataSize);
	ret->data = dataMemory;
	memmove(ret->data, data, dataSize);

	ret->next = NULL;

	return ret;
}

LINKEDLIST* append(LINKEDLIST* last, void* data, size_t dataSize){
	LINKEDLIST* ret = newList(data, dataSize);
	ret->next = last;
	
	return ret;
}

void freeList(LINKEDLIST* list) {
	free(list->data);
	free(list);
}
// 0 = success
// 1 = next is null
int removeNext(LINKEDLIST* curr) {
	if(curr->next){
		LINKEDLIST* toFree = curr-> next;
		curr->next = toFree->next;
		freeList(toFree);
		return 0;
	} else {
		return 1;
	}
}



void printList(LINKEDLIST* list){
	printf("%s§", list->data);
	void* next = list->next;
	if(next){printList(next);}
	else {printf("\n");}
}

char* substring(char* str, int start, int end){
	size_t retSize = (size_t)(end - start + 1);
	char* ret = malloc(sizeof(char) * retSize);

	for(int retI = 0, strI = start; strI < end && str[strI] != '\0'; strI++, retI++){
		ret[retI]=str[strI];
	}
	ret[retSize] = '\0';

	return ret;
}

LINKEDLIST* splitStr(char* str, char delim) {                                                              
	if(str[0] == '\0'){return NULL;}
    // count how many times you have to split                                                      
	int start = 0;
	while(str[start] && str[start] == delim) { start++; }

	if(str[start] == '\0') {return NULL;}
	//printf("weeded trailing delim\n%s\n", str + start);

	size_t elemLenght = 0;
	int end = start;
	while(str[end] && str[end] != delim) {
		end++; elemLenght++;
	}

	if(elemLenght == 0) {return NULL;}
	//printf("Found end of word\n");

	char* substr = substring(str, start, end);

	LINKEDLIST* next = splitStr(str + end, delim);

	LINKEDLIST* curr = newList(substr, elemLenght);
	curr->next = next;
	//printf("succsessfuly created curr node\n%p -- %s\n", next, curr->data); 
	//if(next) { printf("succsessfuly created next node\n%p -- %s\n", next, next->data); }

	return curr;
}
char* readfile(char *path) {
  FILE* f = fopen(path,  "r");
  // f invalid? fseek() fail?
  if (f == NULL || fseek(f, 0, SEEK_END)) {
    return NULL;
  }

  long length = ftell(f);
  rewind(f);
  // Did ftell() fail?  Is the length too long?
  if (length == -1) {
    return NULL;
  }

  // Convert from long to size_t
  size_t ulength = (size_t) length;
  char *buffer = malloc(ulength + 1);
  // Allocation failed? Read incomplete?
  if (buffer == NULL || fread(buffer, 1, ulength, f) != ulength) {
    free(buffer);
    return NULL;
  }
  buffer[ulength] = '\0'; // Now buffer points to a string

  return buffer;
}


int intfromstr(char* str) {
	int ret = 0;
	int strLen = 0;
	while(str[strLen]){ 
		ret *= 10;
		switch(str[strLen]){
			case '9': 
				ret += 1;
			case '8': 
				ret += 1;
			case '7': 
				ret += 1;
			case '6': 
				ret += 1;
			case '5': 
				ret += 1;
			case '4': 
				ret += 1;
			case '3': 
				ret += 1;
			case '2': 
				ret += 1;
			case '1': 
				ret += 1;
			default: break;
		}
		strLen+=1;
	}
	return ret;
}
