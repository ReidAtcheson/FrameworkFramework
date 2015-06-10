ALL : 
	$(CC) -g main.c -o main -DNDEBUG
	$(MAKE) -C examples



clean :
	rm -rf main
	$(MAKE) -C examples clean
