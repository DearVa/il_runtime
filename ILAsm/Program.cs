using System;

namespace TestCsharp {
	public static class Program {
		public static void Main() {
			var a = 1;
			const int b = 2;
			var c = 3;
			
			Console.WriteLine(Add(a + b, c));

			a = (int)BoxAndUnbox(a);

			Add(true, 'a', sbyte.MaxValue, byte.MaxValue, short.MaxValue, ushort.MaxValue, 1, 1);

			var ca = new ClassA();
		}

		private static int Add(int a, int b) {
			return a + b;
		}

		private static object BoxAndUnbox(object a) {
			return a;
		}

		private static void Add(bool a, char b, sbyte c, byte d, short e, ushort f, int g, uint h) {
			Console.WriteLine("8 arguments");
		}
	}

	public class ClassA {

	}
}
