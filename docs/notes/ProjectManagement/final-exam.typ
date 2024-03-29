#set rect(
  width: 100%,
  height: 100%,
  inset: 4pt,
)

#set page(
  number-align: center,
  flipped: true,
  margin: 2mm,
)

#set par(leading: 0.4em,)
#set block(spacing: 0.6em)
#set text(font: "PingFang SC", size: 8pt)

#columns(4, gutter: 4pt)[
  = 牛魔项目管理与产品运维
  == 一、项目管理基本概念
  / 项目管理三要素: 范围、时间、成本
  / 项目: 为创造独特的产品、服务或成果而进行的临时性工作，可以创造产品、能力/提供服务的能力、成果
  - 有明确的结果/目的，为达到预期结果而需要完成的一系列任务；
  - 临时性/一段时间，有明确的开始结尾，*确定的期限*；
  - 资源（人力和其他要素）。
  / 项目集: 一组相互关联且被协调管理的项目，通过产生共同的结果或整体能力而形成相互关系。关注协调关系
  / 项目组合: 为了便于管理、能够实现战略业务目标而被组合在一起的项目、项目集和其他工作项目组合中的项目，不一定彼此依赖或有直接关系。关注优先顺序

  == 二、

  项目经理拥有的权力与 *组织结构* 最相关，组织结构：
  #table(
    columns: (auto, auto, auto, auto, auto, auto),
    inset: 2pt,
    align: horizon,
    [], [*职能型*], [*弱矩阵*], [*平衡矩阵*], [*强矩阵*], [*项目型*],
    [项目经理职权], [很小或没有], [小], [小到中], [中到大], [大到几乎全部],
    [可用的资源], [很少或没有], [少], [少到中], [中到多], [多到几乎全部],
    [项目预算控制者], [职能经理], [职能经理], [混合], [项目经理], [项目经理],
    [项目经理的角色], [兼职], [兼职], [全职], [全职], [全职],
    [项目管理行政人员], [兼职], [兼职], [兼职], [全职], [全职],
  )
  - *职能型* 组织结构最不利于跨多部门的管理
  - *项目型* 组织简单，权责明确
  项目开发生命周期：
  #table(
    columns: (10%, auto, auto, auto),
    inset: 2pt,
    align: horizon,
    [], [*预测型*], [*迭代型* / *增量型*], [*敏捷型*],
    [特点], [按计划执行], [重复循环/渐进增加], [较小增量，快速迭代，每次交付最有价值的东西],
    [需求], [开发前预先确定], [交付期间定期细化], [交付期间频发细化],
    [交付], [一次交付], [分次交付], [频繁交付],
    [关键相关方], [特定里程碑时点参与], [定期参与], [持续参与],
    [优先适用条件], [充分了解产品厚实的行业实践基础整批一次性交付有利于相关方], [不断变化目标和范围需降低项目复杂性部分交付有利于相关方], [需应付快速变化的环境需求和范围难以实现确定较小增量改进有利于相关方]
  )

  == 三、关键因素、过程组、软件及pm
  所有项目通用生命周期：【项目开始点】开始项目【阶段控制关口】组织与准备【阶段控制关口（通用的项目生命周期）】执行项目工作【阶段控制关口】结束项目【项目结束】
  项目生命周期：启动、规划、执行、结束

  == 四、项目整合管理
  / 项目通用的生命周期: 开始项目、组织与准备、执行项目工作、结束项目
  / 项目阶段（五大过程组）: 启动、规划、执行、监控、收尾
  / 启动过程组的管理过程: 指定项目章程、识别相关方
  // #image("assets/项目管理过程.png")

  *人事管理制度* 不是组织过程资产
  === 制定项目章程（启动过程组）
  / 输入: 商业文件、协议、事业环境因素（包含组织文化，不可控、需遵守）、组织过程资产（可裁剪、多积累）
  / 工具与技术: 专家判断、数据收集（头脑风暴、焦点小组、访谈）、人际关系与团队技能（冲突管理、引导、会议管理）、会议
  / 输出: 项目章程（由项目启动者或发起人发布，批准项目成立，授权项目经理动用组织资源，包含项目目的、成目标、成功标准、高层级需求/描述、边界、成果、风险、总体里程碑/预算等）、假设日志（记录假设条件和制约因素）

  === 制定项目管理计划（规划过程组）
  / 输入: 项目章程、*其他过程输出*、事业环境因素、组织过程资产
  / 工具与技术: 同启动过程组
  / 输出: 项目管理计划（需要相关方的一致认可，落实、阐明每个相关方的角色职责，需要经过变更流程的审批才能修改）、项目文件（团队自行维护的，更倾向于工作过程的记录）
  // #columns(2, gutter: 0pt)[
  //   #image("assets/项目管理计划.png")
  //   #colbreak()
  //   #image("assets/项目文件.png")
  // ]

  === 指导与管理项目工作（执行过程组）
  / 输入: 项目管理计划、项目文件、批准的变更请求（更新【改管理计划、基准】、纠正【不改，维护】、预防【不改，维护】、补救【补质量，仅针对质量问题】）、事业环境因素、组织过程资产
  / 工具与技术: 专家判断、项目信息管理系统（自动收集 KPI）、会议
  / 输出: 可交付成果、工作绩效数据、问题日志（问题描述、责任人、解决期限）、变更请求、项目管理计划更新、项目文件更新、组织过程资产更新

  + 实施已计划好的项目活动
  + 管理项目内的各种技术接口和组织接口
  + 回顾所有项目变更的影响，并实施已批准的变更
  + 收集工作绩效数据并传达给合适的控制过程

  === 管理项目知识（执行过程组）
  / 输入: 项目管理计划、项目文件、可交付成果、事业环境因素、组织过程资产
  / 工具与技术: 专家判断、知识管理、信息管理、人际关系与团队技能
  / 输出: 经验教训登记册（项目文件中）、项目管理计划更新、组织过程资产更新
  使用用现有知识并生成新知识，在整个项目期间开展

  === 监控项目工作（监控过程组）
  / 输入: 项目管理计划、项目文件（进度预测、成本预测）、工作绩效信息、协议、事业环境因素、组织过程资产
  / 工具与技术: 专家判断、数据分（挣值分析、偏差分析等）析、决策、会议
  / 输出: 工作绩效报告、变更请求、项目管理计划更新、项目文件更新

  === 实施整体变更控制（监控过程组）
  审查所有变更请求，批准请求，管理变更，对变更处理结果进行沟通，贯穿项目始终，项目经理对此负最终责任，与其他所有过程组都有相互作用。
  / 输入: 项目管理计划（变更管理计划、配置管理计划、范围基准、进度基准、成本基准）、项目文件（估算依据、需求跟踪矩阵、风险报告）、工作绩效报告、变更请求、事业环境因素、组织过程资产
  / 工具与技术: 专家判断、变更控制工具、数据分析、决策、会议
  / 输出: 批准的变更请求，项目管理计划更新、项目文件更新（日志）
  记录 -> 评估 -> 提交 -> 更新 -> 通知

  === 结束项目或阶段（收尾过程组）
  / 输入: 项目章程、项目管理计划、项目文件、验收的可交付成果、商业文件、协议、采购文档、组织过程资产
  / 工具与技术: 专家判断、数据分析、会议
  / 输出: 项目文件更新（经验教训）、最终产品服务或成本移交、最终报告、组织过程资产更新

  == 五、项目范围管理

  收集需求，创建工作分解结构 WBS。

  *项目管理办公室* 管理项目使用的政策、方法、模板


  == 六、项目进度管理（规划过程组）
  活动 + 顺序 + 持续时间 = 进度计划
  包括：规划进度管理、定义活动、排列活动顺序、估算活动持续时间、指定进度计划、控制进度

  === 6.1 规划进度管理
  / 输入: 项目章程、项目管理计划（范围管理计划、开发方法）、事业环境因素、组织过程资产
  / 工具与技术: 专家判断、数据分析、会议
  / 输出: 进度管理计划，项目管理计划的重要组成部分，包含项目进度模型制订（项目活动网络图、关键路径）、进度计划的发布和迭代长度、准确度（估算的可接受区间、允许的应急储备数量）、计量单位（人时、人天等用于资源的计量单位）、组织程序链接、项目进度模型维护、控制临界值、绩效测量规则、报告格式。
  === 6.2 定义活动
  将WBS工作包分解为进度活动
  / 输入: 项目管理计划（进度管理计划、范围基准）、事业环境因素、组织过程资产
  / 工具与技术: 专家判断、分解（把项目范围和项目可交付成果逐步划分为更小、更便于管理的组成部分；WBS中的每个工作包都需要分解成活动，以便通过这些活动来完成相应的可交付成果；让团队成员参与分解过程，有助于得到更好、更准确的结果）、滚动式规划（一种渐进明细的迭代式规划技术，即详细规划近期要完成的 工作，同时在较高层级上粗略规划远期工作；适用于工作包、规划包以及采用敏捷或瀑布式方法的发布规划）、会议
  / 输出: 活动清单（包含项目所需的进度活动、每个活动的标识及工作范围详述， 使项目团队成员知道需要完成什么工作；滚动式规划或敏捷技术需要定期更新）、活动属性（包括唯一活动标识 (ID)、WBS 标识 和活动标签或名称；在活动属性编制完成时，活动属性可能包括活动描述、紧前活动、紧后活动、逻辑关系、提前量和滞后量、资源需求、强制日期、制约因素和假设条件）、里程碑清单（无持续时间，为时点或事件）、变更请求（可能会发现原本不属于项目基准的工作）、项目管理计划更新（进度基准、成本基准）
  === 6.3 排列活动顺序
  / 输入: 项目管理计划（进度管理计划、范围基准）、项目文件（活动属性、活动清单、假设日志、里程碑清单）、事业环境因素、组织过程资产
  / 工具与技术: 紧前关系绘图法（PDM）、确定和整合依赖关系、提前量和滞后量、项目管理信息系统
  / 输出: 项目进度网络图、项目文件更新（活动属性、活动清单、假设日志、里程碑清单）

  PDM 四种逻辑关系：FFFSSFSS
  PDM 四种依赖关系
  - 强制性依赖关系：强制性依赖关系又称硬逻辑关系或硬依赖关系。强制性依赖关系是法律或合同要求的或工作 的内在性质决定的依赖关系，强制性依赖关系往往与客 观限制有关。例如：将数据写入表之前需要创建用户表的位置。
  - 选择性依赖关系：选择性依赖关系有时又称首选逻辑关系、优先逻辑关系或软逻辑关系。选择性依赖关系应基于具体应用领域的最佳实践或项目的某些特殊性质对活 动顺序的要求来创建。例如：先开发浏览器用户登录界 面，还是先开发手机界面，顺序的选择是软件团队的偏 好。
  - 外部依赖关系。外部依赖关系是项目活动与非项目活动之 间的依赖关系，这些依赖关系往往不在项目团队控制范围 内。例如，软件项目的测试活动取决于外部硬件的到货； 托管服务提供商必须先启动虚拟服务器，然后才能安装软 件；有雪的地方才能滑雪。（项目团队不能违反）
  - 内部依赖关系。内部依赖关系是项目活动之间的紧前关系 ，通常在项目团队的控制之中。例如，在开始用户验收测 试之前，必须在QA服务器环境中安装版本发布。软件的安 装由项目组成员完成，这是一个内部的强制性依赖关系。 （项目团队可自由选择）

  - 提前量(lead)是紧后活动可以提前的时间量。（往往表示为 负滞后量，如FS-3）。 
  - 滞后量(lag)是紧后活动必须推迟的时间量。（如FS+2） 
  提前量和滞后量的使用不能替代进度逻辑关系，活动持续时间估算中不包括任何提前量或滞后量。
  === 6.4 估算活动持续时间
  / 输入: 项目管理计划（进度管理计划、项目基准）、项目文件（活动属性、活动清单、假设日志、经验教训登记册、里程碑清单、项目团队派工单、资源分解结构、资源日历、资源需求、风险等级册）、事业环境因素、组织过程资产
  / 工具与技术: 专家判断、类比估算、参数估算、三点估算、自下而上估算、数据分析（备选方案分析、储备分析）、决策、会议
  / 输出: 持续时间估算、估算依据、项目文件更新（活动属性、假设日志、经验教训登记册）

  - 类比估算（自上而下估算）：信息不足、针对整个项目获某个部分、成本较低耗时较小、准确性较低、使用相似活动或项目历史数据。
  - 自下而上估算：最准确
  - 参数估算：基于历史数据和项目参数，准确性取决于参数模型的成熟度和基础数据的可靠性

  ==== 三点估算
  - 最可能时间(Most likely) $"tM"$：基于最可能获得的资源，所估算出来的活动持续时间。
  - 最乐观时间(Optimistic) $"tO"$：基于最好情况所估算的活动持续时间。
  - 最悲观时间(Pessimistic）$"tP"$：基于最差情况所估算的活动持续时间。
  
  三角分布：$"tE" = ("tO" + "tM" + "tP") / 3$

  贝塔（Beta）分布：$"tE" = ("tO" + "4tM" + "tP") / 6$ （pmp考试中默

  - 储备分析：确定项目所需的应急储备量和管理储备。
    - 应急储备是用来应对已经接受的已识别风险。与“已知—未知”风险相关。应急储备在最终的基准中，项目经理可以直接使用，不需要走变更流程。
    – 管理储备是用来应对项目范围中不可预见的风险。与 “未知—未知”风险相关。管理储备不在基准中，项目经理需要走变更流程申请。
  
  - 输出时间估算：对完成某项活动、阶段或项目所需的工作时段数的定量评估
认使用）
  === 6.5 制定进度计划
  / 输入: 进度管理计划（进度管理计划、范围基准）、项目文件（活动属性、活动清单、假设日志、估算一句、持续时间估算、经验教训登记册、里程碑清单、项目进度网络图、项目团队派工单、资源日历、资源请求、风险登记册）、协议、事业环境因素、组织过程资产。
  / 工具与技术: 进度网络分析、关键路径法、资源优化、数据分析（假设情景分析、模拟）、提前量和滞后量、进度压缩、项目管理信息系统、敏捷发布规划
  / 输出: 进度基准、项目进度计划、进度数据、项目日历、变更请求、项目管理计划更新（进度管理计划、成本基准）、项目文件更新（活动属性、假设日志、持续时间估算、经验教训登记册、资源需求、风险登记册）

  ==== 关键路径法
  排出来的进度计划未必可行，关键路径法不考虑资源 约束，需要配合资源平衡处理。
  总浮动时间：活动延期但不至于延误项目完工日期，体现进度灵活性。
  自由浮动时间：活动延期但不延误任何紧后活动最早开始日期。

  ==== 进度压缩
  不缩减项目范围的前提下，缩短或加快工期
  / 赶工: 增加资源，成本/风险增加。只适用于增加资源就能缩短持续时间的、关键路径上的活动。*不计成本地缩短关键路径*
  / 快速跟进: 串行变并行，可能造成返工&风险增加，也可能增加成本。只适用于相互为选择性依赖关系的活动。*没有额外的资源/成本部不能超支*
  优先级：进度压缩 > 赶工 > 快速跟进

  === 6.6 控制进度
  / 输入:
  / 工具与技术:
  / 输出: 


  == 九、项目成本管理
  === 挣值分析（Earned Value Analysis, EVA）
  / 完工预算（Budget at Completion，BAC）: 完成整个计划的预算，即成本基准，同时也是总的 PV

  / 计划价值（Planned Value，PV）: 截止到某个时间点，计划得到的价值，$"PV" = "预算单价" * "计划工作量"$
  / 挣值（Earned Value，EV）: 截止到某个时间点，实际得到的价值，$"EV" = "预算单价" * "实际工作量"$
  / 实际成本（Actual Cost，AC）:截止到某个时间点，实际花费的成本，$"AC" = "实际单价" * "实际工作量"$

  根据上面的指标可以计算出 *进度偏差*（SV）和 *成本偏差*（CV）以及 *进度绩效指数*（SPI）和 *成本绩效指数*（CPI）：

  - SV = EV - PV（挣得=计划，即进度正常，最终会等于 0）

  - CV = EV - AC（挣得=花费，即预算正常）

  - SPI = EV / PV 即 预算时间 / 实际时间
  - CPI = EV / AC 即 预算单价 / 实际单价

  引入一个 *完工估算*（EAC）指标来表示估算完成项目所需的成本，那么便有几种情况：

  - 预算不变：按预算单价完成，$"EAC" = "AC" + ("BAC" - "EV")$
  - 修正预算：按实际单价完成（以当前 CPI 完成工作），$"EAC" = "BAC" / "CPI"$
  - 还有一种引入 SPI 的：$"EAC" = "AC" + ("BAC" - "EV") / ("CPI" * "SPI")$

  还有一个 *完工尚需绩效指数*（TCPI）指标，用于表示为了完成任务，剩余资源的使用必须达到的 CPI（即「还要获取的价值」/「还要花费的成本」）：

  - 按照 BAC 计算，即实现计划预算下完成：$"TCPI" = ("BAC" - "EV") / ("BAC" - "AC")$
  - 按照 EAC 计算，即实现估算预算下完成：$"TCPI" = ("BAC" - "EV") / ("EAC" - "AC")$

  $"TCPI" > 1$ 难以完成，$"TCPI" < 1 $轻易完成。

  == 项目风险管理 
  EMV：正-机会，负-威胁
  决策树
  #image("assets/决策树.png")

  = 牛魔题
  *项目管理计划* 合并与整合了其他各规划过程所输出的所有子计划和基准，在 *规划过程组制定*，整合了各个知识领域的多个管理计划和基准，一旦被确定下来就只有在提出变更请求并被批准后才能变更，是一个渐进明细的过程。

  *收集需求* 过程的输出会醉解作为控制范围的输入。

  *自下而上估算* 方法的准确性主要取决于 *个别计划活动或工作包的规模和复杂程度*

  *商业论证* 文件根据公司在新项目之初所做成本效益分析，阐述了项目是否值得投资

  *根据客户要求变更进度计划* 不属于进度计划控制考虑范围。

  *规划过程组* 制定详细的项目预算

  *项目六个制约因素*：范围、时间、成本、资源、质量、风险，优先关系根据项目需求

  *项目章程* 文件的批准，标志项目的正式启动

  *项目启动会议* 是一个信息沟通和协商的回忆，需要项目在正式投入执行前召开，需要项目相关的各方都参加。

  *蒙特卡洛分析* 可以用来表示项目中存在的风险。

  *收集需求*、*定义范围* 过程的输出会直接影响创建 WBS。

  正确定义了 *等级关系* 的是 战略计划、项目组合、项目集、项目

  *估算成本*：信息来自整体、范围、事件、资源、沟通、风险、采购过程的成果。
  *不正确*：只考虑项目的成本，生命周期成本应当由行政经理来考虑，为了有效控制成本，在项目已开始就应该精确地估算计划活动所需要的费用，为了在项目内或夸项目比较，估算成本必须以货币为单位。

  *项目进度网络图* 属于排列活动顺序过程输出。

  *项目* 和 *运营* 不同是因为当特定目标实现项目就结束了，日常运作是重复进行的。 

  == 2
  根据客户要求变更进度计划 不属于 进度计划控制考虑范围

  已经完成并经实施质量控制过程检验合格的可交付成果是 *确认的可交付成果*

  实施质量控制为 *核实范围* 过程提供确认的可交付成果的输入。

  为了确定客户提出的变更项目范围可能产生的影响，项目经理需要 项目管理计划、需求文件、需求追踪矩阵、组织过程资产及工作绩效信息

  高绩效团队成员离职后，进行计划的质量审计，表明产品质量标准为达到，项目经理应该创建一份因果图。

  *规划风险管理* 和 *规划风险应对* 的主要区别是：前者针对整个项目的风险管理活动，后者针对具体的风险。

  *实施质量保证* 是一个 *执行过程*

  *PDCA*：Plan Do Check Acion

  *风险三要素*：事件、影响、概率

  *龙卷风图*： 不确定的变量与相对稳定的变量之间的相对重要性和相对影响

  队风险进行优先排序的典型方法：*概率和影响矩阵*

  优势、劣势、机会、威胁：SWOT 分析。

  完成工作分解结构，进行估算预算使用 *成本汇总*
  
  == 3
  *问题日志* 用于：记录有谁负责在目标日期内解决特定问题，并监督解决情侣。
  
  *影响项目沟通技术的因素*：项目环境、信息的敏感性、对信息需求的急迫性
  
  50%非口头传播

  团队发展阶段：组织好进展巨大 *成熟*；互相冲突 *震荡*；互相冲突

  *沟通障碍*：兴趣、态度和情绪、偏见

  组织培训的必要信息：资源管理计划

  自组织管理的项目团队：复合型人才

  *沟通渠道多少* 取决于相关方数量

  *RACI图*：用于说明需要完成的工作与团队资源之间的关系

  项目结束，团队解散，丧失士气：收尾要 *提前计划资源的释放*

  *规划沟通管理* 的输入：相关方登记册

  *沟通渠道计算*：$n/2 * (n-1)$

  *马斯洛需求层次理论*： 自我实现 尊重需求 安全需求 生理需求

]