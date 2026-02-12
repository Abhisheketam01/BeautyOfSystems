#include <stdio.h>
#include <time.h>

void slow_part()
{
    volatile long x = 0;
    for (long i = 0; i < 500000000; i++)
    {
        x += i;
    }
}

void fast_part()
{
    volatile long x = 0;
    for (long i = 0; i < 100000000; i++)
    {
        x += i;
    }
}

#include <windows.h>

double now()
{
    static LARGE_INTEGER freq;
    static int initialized = 0;
    LARGE_INTEGER counter;

    if (!initialized)
    {
        QueryPerformanceFrequency(&freq);
        initialized = 1;
    }

    QueryPerformanceCounter(&counter);
    return (double)counter.QuadPart / freq.QuadPart;
}

int main()
{
    double start = now();

    slow_part(); // Optimizable fraction (α)
    fast_part(); // Non-optimizable fraction

    double end = now();
    printf("Execution time: %.3f seconds\n", end - start);
}
