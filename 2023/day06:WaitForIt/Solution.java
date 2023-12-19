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

    private static Stream<String> getAll(String line, Pattern pattern) {
        return pattern.matcher(line).results().map((match) -> match.group());
    }

    private static int conutWays(long time, long dinstance) {
        int count = 0;
        for (int j = 1; j <= time; j++) {
            if ((time-j) * j > dinstance) count++;
        }
        return count;
    }

    private static void part1(List<String> file) {
        Pattern pattern = Pattern.compile("\\d+");
        List<Integer> times = getAll(file.get(0), pattern).map(Integer::parseInt).toList();
        List<Integer> dinstances = getAll(file.get(1), pattern).map(Integer::parseInt).toList();

        int res = 1;
        for (int i = 0; i < times.size(); i++) 
            res *= conutWays(times.get(i), dinstances.get(i));
        
        System.out.println(res);
    }

    private static void part2(List<String> file) {
        Pattern pattern = Pattern.compile("\\d+");
        long time = Long.parseLong(getAll(file.get(0), pattern).reduce((a, b) -> a+b).get());
        long dinstance = Long.parseLong(getAll(file.get(1), pattern).reduce((a, b) -> a+b).get());

        int count = conutWays(time, dinstance);
        
        System.out.println(count); 
    }
}