using System.Runtime.CompilerServices;

namespace TestCsharp {
	public static class Program {
		public static void Main() {
			var a = 1;
			const int b = 2;
			var c = 3;

			c = new StaticAndInstance().Test();

			for (var i = 0; i < 10; i++) {
				if ((i & 1) == 0) {
					WriteLine(i);
				}
			}

			WriteLine(Add(a + b, c));

			a = (int)BoxAndUnbox(a);

			Add(true, 'a', sbyte.MaxValue, byte.MaxValue, short.MaxValue, ushort.MaxValue, 1, 1);

			switch (c) {
			case 1:
				WriteLine(1);
				break;
			case 2:
				WriteLine(2);
				break;
			case 3:
				WriteLine(3);
				break;
			default:
				WriteLine(0);
				break;
			}

			var class0 = new Class0();
			var class1 = new Class1();
			class0 = (Class0)class1;

			object d = "2131231231";
			d = ((string)d).Replace("2", "1");

			var classG = new Class6(a);
			classG.WriteLine(d);
		}

		private static int Add(int a, int b) {
			return a + b;
		}

		private static object BoxAndUnbox(object a) {
			return a;
		}

		private static void Add(bool a, char b, sbyte c, byte d, short e, ushort f, int g, uint h) {
			Program.WriteLine("8 arguments");
		}

#if DEBUG
		[MethodImpl(MethodImplOptions.InternalCall)]
		public static extern void WriteLine(object obj);
#else
		public static void WriteLine(object obj) {
			System.Console.WriteLine(obj);
		}
#endif
	}

	internal class StaticAndInstance {
		/// <summary>
		/// 初始化，使用
		/// </summary>
		public static int staticField1 = 1;
		/// <summary>
		/// 不初始化，使用
		/// </summary>
		public static int staticField2;
		/// <summary>
		/// 初始化，不使用
		/// </summary>
		public static int staticField3 = 3;
		/// <summary>
		/// 不初始化，不使用
		/// </summary>
		public static int staticField4;

		/// <summary>
		/// 初始化，使用
		/// </summary>
		public int instanceField1 = 1;
		/// <summary>
		/// 构造函数中初始化，使用
		/// </summary>
		public int instanceField2;
		/// <summary>
		/// 不初始化，使用
		/// </summary>
		public int instanceField3;
		/// <summary>
		/// 不初始化，不使用
		/// </summary>
		public int instanceField4;

		public StaticAndInstance() {
			instanceField2 = 2;
		}

		public int Test() {
			return staticField1 + staticField2 + instanceField1 + instanceField2 + instanceField3;
		}
	}

	public class Class0 {
		public int A => a;
		protected int a;

		public Class0() {
			a = 0;
		}
	}

	public class Class1 : Class0 {
		protected int b;
		public Class1() {
			a = 1;
		}
	}

	public class Class2 : Class0 {
		protected string c = "c";
	}

	public class Class3 : Class2 {
		protected void WriteLine() {
			Program.WriteLine("Class3");
		}
	}

	public class Class4 : Class3 {
		protected void WriteLine(object s) {
			Program.WriteLine("Class4");
			WriteLine();
		}
	}

	public class Class5 : Class4 { }

	public class Class6 : Class5 {
		public Class6(int a) {
			this.a = a;
		}

		public new void WriteLine(object s) {
			base.WriteLine(s);
			Program.WriteLine(A);
			Program.WriteLine((string)s);
			Program.WriteLine(a);
			Program.WriteLine(c);
		}
	}
}
