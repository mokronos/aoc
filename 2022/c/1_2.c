#include <stdio.h>
#include <stdlib.h>
#include <string.h>


int compare(const void *a, const void *b)
{
    return (*(int *)a - *(int *)b);
}

int main(void)
{
    char *filename = "data1.txt";
    FILE *file = fopen(filename, "r");
    
    const unsigned MAX_LENGTH = 256;
    char buffer[MAX_LENGTH];

    char line[MAX_LENGTH];
    int max_calories[3] = {0};
    int cur_calories = 0;

    while (fgets(line, MAX_LENGTH, file) != NULL)
    {
        if (strcmp(line, "\n") == 0)
        {
            if (cur_calories > max_calories[0])
            {
                max_calories[0] = cur_calories;
            }

            qsort(max_calories, sizeof(max_calories)/sizeof(int), sizeof(int), compare);
            cur_calories = 0;
            continue;
        }
        else
        {
            int calories = atoi(line);
            cur_calories += calories;
        }
    }
    
    fclose(file);

    long sum = 0;
    for (int i = 0, n = sizeof(max_calories) / sizeof(int); i < n; i++)
    {
        sum += max_calories[i];
    }

    printf("Max calories: %li\n", sum);

    return 0;
}


