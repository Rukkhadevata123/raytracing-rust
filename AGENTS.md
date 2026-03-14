# Ray Tracing Rust 项目指南

## 项目概述

这是一个基于 Rust 实现的高性能光线追踪器，参考了 "Ray Tracing in One Weekend" 书籍系列。项目实现了物理渲染中的高级技术，包括 Monte Carlo 路径追踪、重要性采样、BVH 加速结构、体积渲染和程序化纹理。

### 核心特性

- **Monte Carlo 路径追踪**：使用 `PathTracer` 积分器实现基于物理的全局光照
- **重要性采样**：通过概率密度函数（PDF）减少方差，结合光源和 BRDF 采样
- **高级材质系统**：支持介电材质（玻璃）、金属、Lambertian（漫反射）、漫射光源和各向同性体积
- **体积渲染**：支持恒定介质（雾/烟雾）和次表面散射模拟
- **BVH 加速**：使用包围体层次结构实现 O(log n) 的相交性能
- **程序化纹理**：Perlin 噪声、湍流、图像映射和棋盘格图案
- **并行计算**：使用 Rayon 实现多线程渲染管线
- **模块化架构**：通过 `Integrator`、`Material`、`PDF` 和 `Texture` trait 实现清晰的关注点分离

### 技术栈

- **语言**：Rust 2024 Edition
- **数学库**：nalgebra 0.34
- **并行计算**：rayon 1.11
- **图像处理**：image 0.25
- **进度显示**：indicatif 0.18
- **随机数**：rand 0.9

## 项目架构

### 模块结构

```
src/
├── core/              # 核心数学和基础类型
│   ├── camera.rs      # 相机模型（视场、焦点、时间）
│   ├── interaction.rs # 相交交互记录
│   ├── interval.rs    # 数值区间
│   ├── onb.rs         # 正交基（用于局部坐标）
│   ├── ray.rs         # 光线定义
│   └── vec3.rs        # 3D 向量和颜色运算
├── geometry/          # 几何图元和加速结构
│   ├── bvh.rs         # 包围体层次结构
│   ├── constant_medium.rs  # 体积介质
│   ├── hittable_list.rs    # 可相交对象列表
│   ├── hittable.rs    # Hittable trait 定义
│   ├── quad.rs        # 四边形图元
│   ├── sphere.rs      # 球体图元
│   ├── transforms.rs  # 变换（旋转、平移）
│   └── triangle.rs    # 三角形图元
├── integrators/       # 渲染算法
│   ├── integrator_trait.rs  # Integrator trait
│   └── path_tracer.rs       # 路径追踪实现
├── materials/         # 材质系统
│   ├── dielectric.rs       # 玻璃/折射材质
│   ├── diffuse_light.rs    # 发光材质
│   ├── isotropic.rs        # 各向同性体积材质
│   ├── lambertian.rs       # 漫反射材质
│   ├── material_trait.rs   # Material trait
│   └── metal.rs            # 金属材质
├── sampling/          # Monte Carlo 采样
│   ├── pdf.rs         # PDF trait 和实现
│   └── random.rs      # 随机数生成
├── scenes/            # 场景定义
│   ├── cornell_box.rs    # Cornell Box 场景
│   ├── final_scene.rs    # 最终复杂场景
│   └── many_balls.rs     # 随机球体场景
├── textures/          # 纹理系统
│   ├── checker.rs     # 棋盘格纹理
│   ├── image.rs       # 图像纹理
│   ├── noise.rs       # 噪声纹理
│   ├── perlin.rs      # Perlin 噪声实现
│   ├── solid_color.rs # 纯色纹理
│   └── texture_trait.rs   # Texture trait
└── main.rs            # 入口点和场景选择器
```

### 核心设计模式

#### 1. Trait-Based 抽象

项目使用 Rust trait 定义清晰的接口：

- **Hittable**：所有可相交对象（几何图元、光源）的统一接口
- **Material**：材质行为（散射、发光、PDF）的抽象
- **PDF**：概率密度函数，用于重要性采样
- **Texture**：2D/3D 纹理接口
- **Integrator**：渲染算法接口

#### 2. 线程安全共享

使用 `Arc<dyn Trait>` 实现线程安全的共享对象：

```rust
// 示例：共享几何对象
let world: Arc<dyn Hittable> = Arc::new(HittableList::new(objects));

// 示例：共享材质
let material: Arc<dyn Material> = Arc::new(Lambertian::new(color));
```

所有 trait 都要求 `Send + Sync`，确保在多线程环境下的安全性。

#### 3. 渲染方程实现

核心路径追踪算法在 `src/integrators/path_tracer.rs` 中的 `li` 函数实现：

```rust
emission + attenuation * sample_color * scattering_pdf / pdf_val
```

这个公式实现了 Monte Carlo 渲染方程：
- `emission`：光源自发光
- `attenuation`：材质颜色（反照率）
- `sample_color`：入射光颜色
- `scattering_pdf`：材质散射 PDF
- `pdf_val`：采样策略 PDF

当使用完美重要性采样时，`scattering_pdf = pdf_val`，公式简化为 `attenuation * sample_color`。

## 构建和运行

### 环境要求

- Rust 1.70+
- Cargo（随 Rust 安装）

### 构建项目

```bash
# Release 模式（必需，用于性能）
cargo build --release

# Debug 模式（仅用于开发测试）
cargo build
```

### 运行场景

项目支持三个预定义场景，通过命令行参数选择：

```bash
# 场景 1：随机球体（Book 1 最终场景）
cargo run --release -- many_balls

# 场景 2：Cornell Box（Book 3，玻璃球）
cargo run --release -- cornell_box

# 场景 3：最终复杂场景（Book 2 最终场景）
cargo run --release -- final_scene
```

### 输出

渲染的图像将保存为 PNG 文件，文件名与场景名称相同：
- `many_balls.png`
- `cornell_box.png`
- `final_scene.png`

### 性能基准

在 8 核 CPU 上的性能（参考）：

| 场景 | 分辨率 | 样本数 (SPP) | 渲染时间 |
|------|--------|--------------|----------|
| Cornell Box | 600×600 | 1000 | ~5 分钟 |
| Final Scene | 800×800 | 5000 | ~45 分钟 |
| Many Balls | 1200×675 | 500 | ~30 秒 |

在 16 核 CPU 上的性能（参考）：

| 场景 | 分辨率 | 样本数 (SPP) | 渲染时间 |
|------|--------|--------------|----------|
| Cornell Box | 1200×1200 | 10000 | ~2.7 小时 |
| Final Scene | 1200×1200 | 10000 | ~4.5 小时 |
| Many Balls | 1200×675 | 10000 | ~1.5 小时 |

## 开发约定

### 代码风格

1. **模块化设计**：每个模块有明确的职责（核心数学、几何、材质等）
2. **Trait 优先**：使用 trait 定义抽象接口，而非具体类型
3. **线程安全**：所有在多线程中共享的类型必须实现 `Send + Sync`
4. **错误处理**：使用 `Option` 和 `Result` 处理可能的错误情况
5. **命名约定**：
   - 类型：PascalCase（如 `PathTracer`, `HittableList`）
   - 函数/方法：snake_case（如 `calculate_pixel_color`, `scatter`）
   - 常量：SCREAMING_SNAKE_CASE（如 `MAX_DEPTH`）

### 文件组织

- 每个主要模块有自己的目录（如 `geometry/`、`materials/`）
- 模块入口文件（如 `geometry.rs`）导出子模块
- trait 定义在对应的 `*_trait.rs` 文件中
- 实现文件以具体类型命名（如 `lambertian.rs`、`sphere.rs`）

### 数学约定

- 使用 `nalgebra` 进行向量/矩阵运算
- 坐标系：右手坐标系
- 角度：弧度制
- 颜色：RGB 空间，线性颜色
- Gamma 校正：使用 sqrt 函数（gamma = 2.0）

### 渲染管线

渲染过程遵循以下步骤：

1. **相机光线生成**：为每个像素生成多条光线（采样）
2. **并行像素处理**：使用 Rayon 分块渲染（16×16 像素块）
3. **路径追踪积分**：递归计算光照贡献
4. **相交测试**：使用 BVH 加速
5. **材质交互**：计算散射、反射、折射
6. **重要性采样**：混合光源和 BRDF 采样策略
7. **颜色累积**：应用衰减和 PDF 权重
8. **Gamma 校正**：将线性颜色转换为 sRGB
9. **图像保存**：写入 PNG 文件

### 添加新材质

新材质必须实现 `Material` trait：

```rust
use crate::materials::material_trait::{Material, ScatterRecord};
use crate::core::interaction::Interaction;
use crate::core::ray::Ray;
use crate::core::vec3::Color;
use crate::sampling::pdf::PDF;
use std::sync::Arc;

#[derive(Debug)]
pub struct MyMaterial {
    // 材质属性
}

impl Material for MyMaterial {
    fn scatter(&self, r_in: &Ray, isect: &Interaction, srec: &mut ScatterRecord) -> bool {
        // 实现散射逻辑
        true
    }

    fn emitted(&self, r_in: &Ray, isect: &Interaction, u: f64, v: f64, p: &Point3) -> Color {
        // 实现发光（默认为黑色）
        Color::zeros()
    }

    fn scattering_pdf(&self, r_in: &Ray, isect: &Interaction, scattered: &Ray) -> f64 {
        // 返回散射方向的 PDF 值
        0.0
    }
}
```

### 添加新几何图元

新图元必须实现 `Hittable` trait：

```rust
use crate::geometry::hittable::Hittable;
use crate::core::aabb::Aabb;
use crate::core::interval::Interval;
use crate::core::ray::Ray;
use crate::core::interaction::Interaction;

#[derive(Debug)]
pub struct MyPrimitive {
    // 几何属性
}

impl Hittable for MyPrimitive {
    fn hit(&self, r: &Ray, ray_t: Interval, isect: &mut Interaction) -> bool {
        // 实现相交测试
        false
    }

    fn bounding_box(&self) -> Aabb {
        // 返回包围盒
        Aabb::default()
    }

    fn pdf_value(&self, origin: &Point3, direction: &Vec3) -> f64 {
        // 返回从 origin 到 direction 的 PDF 值（用于光源采样）
        0.0
    }

    fn random(&self, origin: &Point3) -> Vec3 {
        // 生成从 origin 到此对象的随机方向
        Vec3::new(1.0, 0.0, 0.0)
    }
}
```

### 添加新场景

在 `src/scenes/` 目录下创建新场景文件：

```rust
use crate::core::camera::Camera;
use crate::core::vec3::Color;
use crate::geometry::hittable_list::HittableList;
use crate::materials::lambertian::Lambertian;
use std::sync::Arc;

pub fn build_my_scene(aspect_ratio: f64, samples: u32, max_depth: u32) -> (Arc<dyn Hittable>, HittableList, Camera) {
    let mut world = HittableList::new();
    let mut lights = HittableList::new();

    // 添加几何对象
    // world.add(Arc::new(Sphere::new(...)));

    // 设置相机
    let lookfrom = Point3::new(0.0, 0.0, 0.0);
    let lookat = Point3::new(0.0, 0.0, -1.0);
    let vup = Vec3::new(0.0, 1.0, 0.0);
    let dist_to_focus = 10.0;
    let aperture = 0.0;
    let fov = 90.0;

    let camera = Camera::new(
        lookfrom, lookat, vup, fov, aspect_ratio, aperture, dist_to_focus, 0.0, 1.0, Color::zeros()
    );

    (Arc::new(world), lights, camera)
}
```

然后在 `src/main.rs` 中注册新场景：

```rust
use crate::scenes::my_scene::build_my_scene;

fn main() {
    // ... 现有代码 ...

    let (world, lights, camera) = match scene_name {
        // ... 现有场景 ...
        "my_scene" => {
            println!("Loading My Scene...");
            build_my_scene(1200.0 / 675.0, 10000, 75)
        }
        _ => { /* ... */ }
    };

    // ... 其余代码 ...
}
```

## 测试和调试

### 验证渲染结果

由于这是一个图形渲染项目，"测试"主要通过视觉验证：

1. 运行场景生成图像
2. 与预期结果比较（参考 `images/` 目录）
3. 检查是否有明显的渲染错误（噪声、伪影、光照问题）

### 性能分析

使用 Release 模式进行性能测试：

```bash
cargo build --release
time cargo run --release -- cornell_box
```

### 调试技巧

- 降低样本数（SPP）加快迭代：修改场景构建函数中的 `samples` 参数
- 降低分辨率：修改 `aspect_ratio` 和相机设置
- 使用 Debug 模式验证逻辑：`cargo run -- -- cornell_box`（会慢很多）
- 检查 NaN/Inf：代码中已有 `is_finite()` 检查，可以添加更多调试输出

## 常见问题

### 渲染太慢

- 确保使用 `--release` 模式
- 降低样本数（SPP）
- 降低分辨率
- 减少场景中的对象数量
- 简化材质（避免复杂的程序化纹理）

### 图像太暗

- 增加光源强度
- 增加样本数（减少噪声）
- 检查材质的衰减值是否过低
- 检查光源是否被遮挡

### 噪声问题

- 增加样本数（SPP）
- 改进重要性采样策略
- 检查 PDF 实现是否正确
- 确保使用混合 PDF（光源 + BRDF）

### 编译错误

- 确保 Rust 版本 >= 1.70
- 检查 Cargo.toml 依赖版本
- 清理构建缓存：`cargo clean && cargo build --release`

## 参考资料

- [Ray Tracing in One Weekend](https://raytracing.github.io/books/RayTracingInOneWeekend.html)
- [Ray Tracing: The Next Week](https://raytracing.github.io/books/RayTracingTheNextWeek.html)
- [Ray Tracing: The Rest of Your Life](https://raytracing.github.io/books/RayTracingTheRestOfYourLife.html)
- [Physically Based Rendering](http://www.pbr-book.org/)
- [Rust Book](https://doc.rust-lang.org/book/)
- [Rayon 文档](https://docs.rs/rayon/)

## 许可证

MIT License