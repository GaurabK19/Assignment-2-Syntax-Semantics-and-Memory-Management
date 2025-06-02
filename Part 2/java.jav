public class GarbageCollectionDemo {
    public static void main(String[] args) {
        for (int i = 0; i < 10000; i++) {
            String temp = new String("MemoryTest " + i); // Allocate memory
        } // No explicit deletion necessary

        // Garbage collection
        System.gc();
        System.out.println("Garbage collection requested.");
    }
