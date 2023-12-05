#include <math.h>
#include <stdio.h>
#include </home/etonit/advent-of-code2023/util.c>

/*
winning numbers | numbers you have
each number you have that is in the winning numbers dubles the value starting from one
sum of all points
*/

int main(int argc, char** argv) {
	/*
	char* strTest = "1234567890 oh no why";
	printf("%s\n", substring(strTest, 10, 100));
	LINKEDLIST* split = splitStr(strTest, '\n');
	printf("succsessful split\n");
	printList(split);
	printf("succsessful print\n");
	*/

	void* a = NULL;
	while(a){ return 1;}

	char* input = readfile("/home/etonit/advent-of-code2023/4day/input");
	LINKEDLIST* lines = splitStr(input, '\n');
	printList(lines);
	printf("read whole file\n");
	
	LINKEDLIST* currNode = lines;

	int tot = 0;
	int copies[219];
	for(int i = 0; i < 219; i++) {copies[i] = 1;}
	int copiIdx = 0;
	while(currNode){
		char* currLine = currNode->data;

		int i = 0;
		while(currLine[i] != ':'){i++;}
		int end = i;
		while(currLine[end] != '\0'){end++;}

		char* numbers = substring(currLine, i + 1, end);

		int divider = 0;
		while(numbers[divider] != '|') {divider++;}

		LINKEDLIST* winningNums = splitStr(substring(numbers, 0, divider), ' ');
		LINKEDLIST* myNums = splitStr(substring(numbers, divider + 1, end-i), ' ');
		printList(winningNums);
		printList(myNums);

		int matches = 0;
		LINKEDLIST* currWinning = winningNums;
		while(currWinning){
			//printf("%s\n", currWinning->data);
			int wn = intfromstr(currWinning->data);
			//printf("%i\n", wn);

			LINKEDLIST* currMine = myNums;
			while(currMine){
				int on = intfromstr(currMine->data);
				if(wn == on) { matches += 1; }
				currMine = currMine->next;
			}

			currWinning = currWinning->next;
		}
		int points = matches == 0 ? 0 : 1;
		for(int m = matches;m> 1; m--){ points *= 2; }
		tot += points;

		for(int i = copiIdx + 1; i <= copiIdx + matches; i++){
			copies[i] += copies[copiIdx];
		}

		copiIdx++;
		currNode = currNode->next;
	}

	int acc = 0;
	for(int i = 0; i < 219; i++){acc += copies[i];};
	printf("tot points = %i\n", tot);
	printf("tot cards = %i\n", acc);

	return 0;
}
