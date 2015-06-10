ALL : 
	$(CC) main.c -o main
	$(MAKE) -C examples



clean :
	rm -rf main
	$(MAKE) -C examples clean
