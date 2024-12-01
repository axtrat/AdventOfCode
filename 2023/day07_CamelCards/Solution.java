import java.io.File;
import java.io.FileNotFoundException;
import java.util.ArrayList;
import java.util.List;
import java.util.Scanner;
import java.util.regex.Pattern;
import java.util.stream.Stream;

/**
 * Solution
 */
public class Solution {

    public static void main(String[] args) throws FileNotFoundException {
        String fileName = (args.length == 0) ? "input" : args[0];
        List<String> lines = new ArrayList<>();
        try (Scanner sc = new Scanner(new File(fileName + ".txt"))) {
            while (sc.hasNextLine()) {
                lines.add(sc.nextLine());
            }
        }
        MyTimer timer = new MyTimer();
        part1(lines);
        System.out.println(timer);
        timer.restart();
        part2(lines);
        System.out.println(timer);
    }

    private static void part1(List<String> file) {
        System.out.println("Not implemented");
    }

    private static void part2(List<String> file) {
        System.out.println("Not implemented");
    }
}