import { CSSProperties } from "react";

const FlowLogo: React.FC<{ className?: string; id?: string; style?: CSSProperties }> = ({
  className,
  id,
  style,
}) => (
  // <svg
  //   id={id}
  //   className={className}
  //   // width="92"
  //   // height="102"
  //   viewBox="0 0 92 102"
  //   fill="none"
  //   style={style}
  //   xmlns="http://www.w3.org/2000/svg">
  //   <path
  //     d="M91.9993 68.3491C91.8922 68.5641 91.785 68.7823 91.6747 69.0005C89.7273 72.6439 87.7713 76.2874 85.8066 79.9308C85.7677 80.0024 85.732 80.0773 85.6866 80.1457C85.6253 80.2474 85.5405 80.3327 85.4393 80.3945C85.3382 80.4563 85.2237 80.4928 85.1056 80.5007H7.12013C6.96434 80.5069 6.81014 80.4674 6.6764 80.387C6.54267 80.3066 6.43519 80.1888 6.36715 80.048C4.30078 76.2678 2.23116 72.4844 0.158298 68.6976C0.0511219 68.5312 -0.00588989 68.3372 -0.00588989 68.1391C-0.00588989 67.9409 0.0511219 67.747 0.158298 67.5805C5.19981 58.8107 10.2316 50.0365 15.2536 41.258C23.002 27.7287 30.7482 14.1929 38.4922 0.650643C38.5949 0.437649 38.7599 0.261172 38.9652 0.144779C39.1705 0.0283859 39.4063 -0.0223811 39.6411 -0.000742213C43.7306 0.0187994 47.8168 -0.000742213 51.9063 -0.000742213H52.1497C52.3037 -0.0103527 52.4571 0.0255618 52.5909 0.102555C52.7247 0.179548 52.8331 0.294239 52.9027 0.432429C53.1818 0.920968 53.4631 1.40734 53.7465 1.89153L77.764 43.3197C82.4312 51.3708 87.1016 59.4197 91.7753 67.4665C91.8694 67.6261 91.9441 67.7922 92.0252 67.9583L91.9993 68.3491ZM89.9188 68.9387H26.7236C26.6093 68.9361 26.4969 68.9082 26.3947 68.8568C26.2924 68.8054 26.2028 68.7318 26.1323 68.6415C26.0618 68.5511 26.0122 68.4462 25.9871 68.3342C25.9619 68.2223 25.9618 68.1061 25.9868 67.9941C26.0257 67.8534 26.0836 67.7186 26.1589 67.5935C28.1582 64.1542 30.1607 60.7138 32.1665 57.2723L32.2963 57.0411L32.186 56.8652C30.4643 54.3036 29.4866 51.3119 29.3623 48.2246C29.3249 47.3123 29.3607 46.3985 29.4694 45.492C29.6272 44.1144 29.9632 42.7634 30.469 41.473C31.4861 38.8782 33.1387 36.5827 35.2739 34.7986C37.4092 33.0145 39.9583 31.7994 42.6855 31.2658L43.036 31.1876C42.7894 30.6437 39.6768 25.1916 39.5373 25.0613C29.1751 43.0048 18.8086 60.9668 8.43784 78.9472H84.1871C84.2356 78.9423 84.2846 78.9423 84.3331 78.9472C84.5084 78.9765 84.5928 78.8853 84.6577 78.742C86.3649 75.5546 88.0742 72.3693 89.7858 69.1862C89.8377 69.1178 89.8669 69.0461 89.9188 68.9387ZM7.09416 78.1167L7.2532 77.8561C9.20056 74.4624 11.1555 71.0687 13.118 67.675C16.4588 61.8885 19.8007 56.1009 23.1437 50.3123L38.8135 23.1723C38.8492 23.1104 38.8849 23.0453 38.9271 22.9834C38.9981 22.8759 39.0946 22.7877 39.2079 22.7267C39.3211 22.6657 39.4477 22.6338 39.5762 22.6338C39.7048 22.6338 39.8313 22.6657 39.9446 22.7267C40.0578 22.7877 40.1543 22.8759 40.2253 22.9834C40.2757 23.0574 40.3213 23.1347 40.3616 23.2146L44.6036 30.7447L44.7465 30.9824C48.8046 30.6971 52.8254 31.9215 56.0412 34.4217C59.2738 36.8975 61.4963 40.4707 62.2927 44.4724C63.0891 48.4741 62.4046 52.6299 60.3676 56.1617H70.5361C60.1306 38.2204 49.7295 20.2856 39.3328 2.35727C39.2484 2.50058 39.1867 2.6048 39.1283 2.70902C27.323 23.3254 15.5144 43.9417 3.7025 64.5581C3.05338 65.6882 2.40426 66.8379 1.75514 67.9518C1.7109 68.0171 1.68725 68.0943 1.68725 68.1733C1.68725 68.2523 1.7109 68.3294 1.75514 68.3947C3.48613 71.5496 5.21712 74.7078 6.94811 77.8691C6.9903 77.9375 7.0325 78.0059 7.09416 78.1167ZM56.9532 53.364C56.2128 51.8354 55.2885 50.4036 54.2009 49.1007C52.8895 50.3924 51.4295 51.5229 49.8518 52.4684L51.955 57.0932C53.8764 56.1682 55.603 55.012 56.9532 53.3575V53.364ZM51.0819 57.4612C50.3938 55.9435 49.7057 54.4388 49.0112 52.9015C47.2724 53.736 45.4081 54.2763 43.4937 54.5007C43.84 56.0848 44.3355 57.6323 44.9737 59.1223C46.5315 58.9887 50.2121 57.9726 51.0819 57.4547V57.4612ZM42.4453 47.7817C42.6174 49.7251 42.8981 51.6573 43.2859 53.5692C45.1358 53.372 46.9377 52.8552 48.612 52.0417L46.0479 46.7655C44.8963 47.2588 43.6851 47.5982 42.4453 47.7751V47.7817ZM53.5778 48.3516C52.3687 46.9558 51.0825 45.6292 49.7252 44.3782C48.8755 45.1577 47.9285 45.8234 46.908 46.3584L49.4364 51.5695C50.2186 51.2828 53.0844 49.0453 53.5778 48.3451V48.3516ZM43.9383 59.2102C43.6722 58.4416 43.3963 57.699 43.1626 56.9304C42.9289 56.1617 42.7309 55.3833 42.5135 54.6212C41.6599 54.5951 40.8095 54.6049 39.9624 54.5333C39.1153 54.4616 38.2845 54.315 37.4341 54.2076C37.4341 54.3737 37.4341 54.5495 37.4568 54.7254C37.4925 55.2498 37.5023 55.7807 37.5802 56.3018C37.6966 57.0167 37.8869 57.7174 38.1481 58.3927C38.1678 58.445 38.2005 58.4913 38.2431 58.5273C38.2857 58.5632 38.3369 58.5875 38.3916 58.5979C39.1484 58.8135 39.9211 58.9682 40.7024 59.0604C41.2899 59.1255 41.8806 59.1744 42.4713 59.2037C42.9646 59.22 43.4515 59.2037 43.9383 59.2037V59.2102ZM38.0962 47.6058C37.6972 49.4362 37.4797 51.3017 37.4471 53.1751C37.512 53.1947 37.5574 53.2142 37.6029 53.224C39.0936 53.5581 40.6199 53.7048 42.1467 53.6604C42.2116 53.6515 42.2756 53.6373 42.3382 53.6181C41.9617 51.716 41.6891 49.8205 41.5138 47.8859C40.3658 47.9547 39.2138 47.8582 38.093 47.5993L38.0962 47.6058ZM56.7585 43.6975C55.346 42.5943 53.8062 41.6659 52.1724 40.9323C51.7418 41.9534 51.1403 42.893 50.3938 43.7105C51.7491 44.968 53.0372 46.2965 54.2528 47.6904C55.3197 46.5141 56.1673 45.155 56.7552 43.6779L56.7585 43.6975ZM57.5147 44.3716C56.8626 45.8519 55.9735 47.215 54.8825 48.407C55.3758 49.0844 55.8562 49.726 56.3268 50.4002C56.7974 51.0744 57.1901 51.7812 57.625 52.4781L57.6932 52.3869C58.6773 50.979 59.4697 49.4456 60.0495 47.8272C60.0695 47.7743 60.0772 47.7174 60.0722 47.661C60.0671 47.6046 60.0494 47.5501 60.0203 47.5016C59.4198 46.3909 58.6462 45.3838 57.7289 44.5182C57.6607 44.4563 57.5926 44.414 57.5114 44.3521L57.5147 44.3716ZM34.8279 46.0978C34.8042 46.1213 34.7835 46.1475 34.7662 46.176C34.6656 46.3779 34.565 46.5831 34.4709 46.7915C33.8835 48.0515 33.4707 49.3862 33.244 50.7585C33.2328 50.8177 33.237 50.8789 33.2564 50.9359C33.2757 50.993 33.3095 51.0441 33.3544 51.0842C34.1947 51.9181 35.2203 52.5398 36.3468 52.8983C36.3975 52.9038 36.4487 52.9038 36.4994 52.8983C36.5544 51.0356 36.7815 49.1821 37.1777 47.3615C36.3297 47.0635 35.5352 46.6302 34.8246 46.0783L34.8279 46.0978ZM52.6463 37.1119C52.6657 37.62 52.7144 38.089 52.695 38.5743C52.6755 39.0596 52.5781 39.5514 52.5164 40.0399C54.1236 40.7716 55.6471 41.6758 57.0603 42.7367C57.2265 42.1418 57.3245 41.5299 57.3524 40.9128C57.3774 40.2213 57.3502 39.5288 57.2713 38.8414C57.2678 38.7824 57.2485 38.7256 57.2153 38.6768C57.1822 38.628 57.1364 38.5892 57.083 38.5645C55.7299 37.8887 54.2905 37.4032 52.8053 37.1217C52.7524 37.1064 52.698 37.0965 52.643 37.0924L52.6463 37.1119ZM51.4714 58.3699C49.5319 59.1603 47.507 59.7199 45.4378 60.0375C45.467 60.1059 45.48 60.1515 45.5027 60.1938C45.9033 60.9579 46.3924 61.6717 46.96 62.3206C46.9886 62.355 47.0263 62.3806 47.0688 62.3944C47.1113 62.4083 47.1568 62.4098 47.2001 62.3987C47.6123 62.3238 48.031 62.2652 48.4465 62.1838C49.8008 61.9229 51.1206 61.5065 52.3801 60.9429C52.4431 60.9137 52.5038 60.88 52.5619 60.8419L51.4714 58.3699ZM52.3412 57.9889L53.4025 60.4381L53.5031 60.399L53.6557 60.3241C55.1448 59.504 56.4972 58.4557 57.664 57.217C57.7262 57.1558 57.7662 57.0756 57.7776 56.989V56.3865C57.7451 55.7614 57.6249 55.1441 57.4206 54.5528C57.4043 54.5039 57.3784 54.4583 57.3459 54.3834C55.9291 55.8999 54.227 57.1198 52.3379 57.9726L52.3412 57.9889ZM44.4024 60.1482C43.455 60.201 42.5052 60.1901 41.5593 60.1156C40.6267 60.0368 39.6992 59.9063 38.781 59.7248C38.931 60.0079 39.0957 60.283 39.2744 60.5488C39.7279 61.2361 40.3085 61.8298 40.9848 62.2978C41.0904 62.3701 41.2117 62.4159 41.3386 62.4313C42.1532 62.5062 42.9614 62.5876 43.7858 62.617C44.4349 62.6397 45.0645 62.5942 45.7072 62.5746C45.764 62.5648 45.8196 62.5484 45.8727 62.5258C45.294 61.7843 44.7998 60.9803 44.3992 60.1287L44.4024 60.1482ZM42.1013 41.1701C42.1337 43.0754 42.2051 44.9546 42.3674 46.8274C43.1464 46.785 45.1619 46.2118 45.613 45.8894L43.049 40.9258L42.1013 41.1701ZM33.1759 52.2078L33.111 52.2371C33.1337 52.6573 33.1499 53.0807 33.1791 53.5008C33.2454 54.3027 33.4089 55.0934 33.666 55.8556C33.7183 56.017 33.8164 56.1596 33.9483 56.2659C34.7217 56.9231 35.5812 57.471 36.5026 57.8944C36.6714 57.9726 36.8466 58.041 37.0219 58.1126L37.0609 58.0768C36.574 56.7415 36.5253 55.3442 36.4896 53.9275C35.2745 53.588 34.1443 52.9954 33.1726 52.1883L33.1759 52.2078ZM60.4714 46.4072C60.4974 46.3225 60.5039 46.2932 60.5136 46.2607C60.7482 45.3654 60.9025 44.4509 60.9745 43.5281C60.9903 43.3509 60.975 43.1723 60.9291 43.0005C60.6949 42.2395 60.3996 41.4989 60.0462 40.7858C60.0028 40.6809 59.9375 40.5865 59.8548 40.5089C59.4004 40.1311 58.9395 39.7598 58.4786 39.3918C58.4159 39.3501 58.3509 39.3121 58.2839 39.2778C58.415 40.6622 58.2727 42.0589 57.8652 43.3881C58.8776 44.2516 59.755 45.2627 60.4682 46.3877L60.4714 46.4072ZM41.4165 46.899C41.4165 46.7981 41.3905 46.6971 41.384 46.5929C41.3483 46.0359 41.3094 45.4757 41.2769 44.9156C41.2444 44.3554 41.212 43.7333 41.1925 43.1405C41.1925 42.6748 41.1925 42.2123 41.1698 41.7466C41.1698 41.5707 41.1406 41.3981 41.1244 41.2189L40.6765 41.1668L40.2351 41.1049C39.43 42.8888 38.787 44.7419 38.3137 46.6417C38.7486 46.8502 40.7641 47.0456 41.4165 46.899ZM45.3112 39.3462L49.7155 43.033C50.41 42.3035 51.1663 41.0496 51.2799 40.522C49.4898 39.7667 47.6453 39.1483 45.7623 38.672L45.3112 39.3462ZM49.0079 43.7333C48.9421 43.6636 48.8728 43.5972 48.8002 43.5346C47.9109 42.7683 47.0162 42.0084 46.1161 41.2548C45.6617 40.8737 45.1911 40.5154 44.727 40.1441C44.6653 40.0953 44.6036 40.0269 44.5128 40.0888L43.8961 40.522L46.4699 45.5018C47.3916 45.0301 48.2454 44.4352 49.0079 43.7333ZM51.6044 39.6133C51.8333 38.7312 51.8333 37.8051 51.6044 36.923C50.6957 36.6723 46.6711 36.5778 46.1421 36.7765V37.7796C48.0026 38.2645 49.8277 38.8772 51.6044 39.6133ZM32.3904 49.8303H32.4489C32.6051 49.0522 32.8289 48.2894 33.1174 47.5504C33.4063 46.8209 33.7341 46.1043 34.0522 45.3715C33.4458 44.7355 32.9772 43.9803 32.676 43.1536C32.6565 43.1986 32.6338 43.2421 32.6079 43.2838C32.0381 43.9631 31.5611 44.7155 31.1896 45.5213C31.1503 45.6251 31.1359 45.7367 31.1474 45.847C31.1474 46.0555 31.1993 46.2639 31.2285 46.4724C31.362 47.4572 31.6559 48.4134 32.0983 49.3026C32.1892 49.5046 32.2931 49.6707 32.3904 49.8303ZM56.9954 37.392C56.9013 37.1478 56.8266 36.9263 56.7325 36.7113C56.5174 36.1714 56.1594 35.7006 55.6971 35.35C55.3953 35.1317 55.1032 34.9005 54.7981 34.6986C54.6773 34.6181 54.549 34.5494 54.4151 34.4934C53.3373 34.057 52.1822 33.8454 51.0202 33.8713C50.9718 33.8767 50.924 33.8865 50.8774 33.9006C51.4713 34.4828 51.9397 35.1812 52.2535 35.9525C52.2957 36.0534 52.3769 36.0632 52.4678 36.0795C53.4035 36.2343 54.3259 36.4619 55.2265 36.7602C55.8237 36.9524 56.3787 37.1771 56.9954 37.4051V37.392ZM37.3919 46.3844C37.8787 44.4579 38.5303 42.5771 39.3393 40.763L38.794 40.4373C37.4458 41.8962 36.2616 43.4996 35.2628 45.2184C35.9005 45.7301 36.6205 46.1289 37.3919 46.3975V46.3844ZM58.0437 53.4813C58.3632 54.2711 58.5813 55.0985 58.6928 55.9435C58.735 55.9175 58.7545 55.9109 58.761 55.9012C58.8292 55.8197 58.8973 55.7351 58.959 55.6536C59.6199 54.7578 60.2154 53.8152 60.7408 52.8331C60.9271 52.5005 61.0202 52.1235 61.0102 51.7421C61.0071 51.181 60.9583 50.6211 60.8641 50.068C60.7992 49.713 60.7084 49.3645 60.624 48.9932C60.2248 49.7814 59.8515 50.5435 59.4523 51.2959C59.0343 52.0607 58.5637 52.7953 58.0437 53.4943V53.4813ZM34.5617 44.528C35.5812 42.8079 36.7778 41.1999 38.1319 39.7305L37.9307 39.3723C37.9307 39.3527 37.9047 39.3332 37.8852 39.3006C36.2408 40.0655 34.7356 41.1014 33.4323 42.3654C33.6241 43.173 34.0122 43.9204 34.5617 44.541V44.528ZM51.176 35.8645C51.1607 35.8246 51.1434 35.7854 51.1241 35.7473C50.7571 35.1041 50.2687 34.5388 49.6863 34.083C49.6404 34.0484 49.5879 34.0238 49.5319 34.0109C49.476 33.998 49.418 33.997 49.3617 34.0081C48.4999 34.161 47.6544 34.3955 46.8366 34.7083C46.3758 34.8842 45.9279 35.0829 45.4572 35.2783C45.5644 35.4379 45.6585 35.5714 45.7461 35.705C45.7672 35.7348 45.7967 35.7576 45.8309 35.7703C45.865 35.7831 45.9022 35.7853 45.9376 35.7766C46.4536 35.7506 46.9697 35.7245 47.489 35.7115C48.0083 35.6984 48.4627 35.7115 48.9495 35.7115C49.3455 35.7115 49.7382 35.7538 50.1342 35.7799C50.462 35.8255 50.7995 35.8483 51.176 35.8776V35.8645ZM30.508 47.723H30.4463C30.4043 47.9525 30.3751 48.1841 30.3587 48.4167C30.3576 48.6758 30.3728 48.9347 30.4041 49.1919C30.46 49.8391 30.5686 50.4807 30.7287 51.1102C31.0301 52.2363 31.5207 53.3025 32.1795 54.2629C32.2119 54.3118 32.2541 54.3509 32.3223 54.4323C32.3223 54.2922 32.3223 54.2043 32.2996 54.1066C32.2574 53.6083 32.1989 53.1067 32.1795 52.6052C32.1795 52.2339 32.2054 51.8593 32.2184 51.488C32.2304 51.4203 32.2288 51.3509 32.2138 51.2838C32.1987 51.2167 32.1705 51.1532 32.1308 51.0972C31.4658 50.253 30.9679 49.289 30.6638 48.2572C30.5989 48.0813 30.5534 47.9022 30.508 47.723ZM38.1189 36.5713C38.0689 36.5568 38.0179 36.5459 37.9664 36.5387C37.5023 36.4834 37.0349 36.4378 36.5708 36.3791C36.1914 36.3493 35.8099 36.3668 35.4348 36.4313C35.3789 36.4381 35.3251 36.4564 35.2765 36.485C35.2279 36.5135 35.1857 36.5518 35.1524 36.5974C34.7077 37.1251 34.3244 37.7021 34.01 38.317C33.9857 38.3762 33.9662 38.4371 33.9516 38.4994C34.3118 38.3366 34.6396 38.1737 34.9739 38.0369C35.8449 37.6819 36.7482 37.4125 37.671 37.2325C37.748 37.224 37.8193 37.1881 37.8723 37.1315C37.9729 36.9556 38.028 36.7667 38.1124 36.5713H38.1189ZM41.3645 38.1086C41.6573 38.1227 41.9491 38.0638 42.2136 37.9371C42.4781 37.8103 42.7072 37.6197 42.8802 37.3823C42.9598 37.2907 43.0136 37.1795 43.0361 37.0602C43.0586 36.9408 43.0491 36.8176 43.0084 36.7032C42.9678 36.5887 42.8975 36.4873 42.8048 36.4091C42.7121 36.331 42.6004 36.2791 42.481 36.2586C42.0641 36.1385 41.617 36.1865 41.2347 36.3922C40.9392 36.5223 40.699 36.753 40.5564 37.0436C40.5062 37.1254 40.4747 37.2173 40.4641 37.3128C40.4536 37.4083 40.4642 37.505 40.4952 37.5959C40.5262 37.6868 40.5769 37.7696 40.6436 37.8386C40.7103 37.9075 40.7913 37.9608 40.8809 37.9946C41.0394 38.0425 41.201 38.0795 41.3645 38.1053V38.1086ZM38.7875 61.5584L38.8265 61.5129C38.3392 60.8904 37.9284 60.2113 37.6029 59.4903C37.5783 59.4279 37.5397 59.3719 37.4902 59.3268C37.4407 59.2816 37.3815 59.2484 37.3173 59.2297C36.6443 59.0022 35.9927 58.7156 35.3699 58.3732L34.6851 57.9856L34.6461 58.0214C34.8506 58.3308 35.0551 58.6435 35.2628 58.9497C35.332 59.0597 35.4137 59.1613 35.5062 59.2525C36.1047 59.802 36.7473 60.3009 37.4276 60.7442C37.882 61.0373 38.3364 61.2979 38.7875 61.5552V61.5584ZM33.27 41.1277L33.3414 41.1636L33.7244 40.8379C34.8396 39.9202 36.0586 39.1375 37.3562 38.5059L37.6159 38.3724C37.684 38.3366 37.7035 38.2844 37.6516 38.2063C37.6213 38.2047 37.591 38.2047 37.5607 38.2063C36.8722 38.3545 36.1962 38.5559 35.5387 38.8088C34.8585 39.0681 34.2061 39.3954 33.5913 39.7859C33.5102 39.8413 33.429 39.8869 33.4128 40.0008C33.3641 40.3852 33.3122 40.7662 33.2635 41.1375L33.27 41.1277ZM48.2517 33.2623C48.229 33.2362 48.2193 33.2199 48.2063 33.2134C47.6428 32.9652 47.0541 32.7795 46.4504 32.6597C46.4068 32.6527 46.3622 32.6558 46.32 32.6688C46.2778 32.6818 46.2392 32.7043 46.207 32.7346C45.5384 33.2176 44.9169 33.7628 44.3505 34.3631C44.3291 34.3911 44.3096 34.4205 44.2921 34.451C44.3408 34.4771 44.3829 34.4999 44.4219 34.5259C44.4789 34.5706 44.5492 34.5948 44.6215 34.5948C44.6938 34.5948 44.7641 34.5706 44.8211 34.5259C44.9338 34.4621 45.0509 34.4066 45.1716 34.3598C46.0571 33.9379 46.9781 33.5957 47.9239 33.3372C48.0343 33.3437 48.1284 33.3046 48.2452 33.272L48.2517 33.2623ZM36.415 35.3369V35.3955L36.6487 35.4183C36.7525 35.4183 36.8596 35.4183 36.9732 35.4379C37.5347 35.5193 38.093 35.6072 38.6545 35.6854C38.7145 35.6975 38.7768 35.6871 38.8297 35.6561C39.0407 35.4867 39.2452 35.3044 39.4788 35.1089C39.176 34.7002 38.8406 34.3168 38.4759 33.9625C37.7386 34.347 37.0454 34.8112 36.4085 35.3467L36.415 35.3369ZM39.3685 33.5456L40.2967 34.6465L41.3548 34.2817L41.7573 32.764C40.9369 32.9436 40.1367 33.2055 39.3685 33.5456ZM44.938 32.4839H44.0779C43.802 32.4839 43.4287 32.5197 43.1042 32.5425C43.0641 32.5383 43.0236 32.5456 42.9875 32.5634C42.9513 32.5813 42.9209 32.609 42.8997 32.6434C42.6374 33.1111 42.4594 33.6216 42.3739 34.1514L43.221 34.1937L44.938 32.4839ZM32.3223 40.8183C32.2703 40.8607 32.2314 40.8835 32.1989 40.916C31.9717 41.1701 31.7446 41.4241 31.5141 41.6749C31.4009 41.7915 31.3301 41.9429 31.3129 42.1048C31.2869 42.3686 31.2285 42.6324 31.1928 42.8963C31.1571 43.1601 31.1506 43.3718 31.1311 43.6095C31.1503 43.6075 31.1688 43.6016 31.1856 43.5921C31.2023 43.5825 31.217 43.5696 31.2285 43.5542C31.5985 43.0949 31.9717 42.6324 32.3353 42.17C32.3769 42.1123 32.3986 42.0425 32.3969 41.9713C32.3774 41.6 32.3482 41.2287 32.3223 40.8183ZM51.2539 32.8942C50.1915 32.4767 49.078 32.2039 47.9434 32.0833C47.9716 32.1041 48.0021 32.1216 48.0343 32.1354C48.6062 32.3461 49.1554 32.6145 49.6733 32.9366C49.7352 32.9725 49.8063 32.9895 49.8778 32.9854C50.3224 32.9561 50.7638 32.9235 51.2539 32.8942ZM61.536 46.1011C61.4029 46.5701 61.2698 47.0391 61.14 47.5081C61.1222 47.562 61.1222 47.6202 61.14 47.6742C61.2893 48.0845 61.4451 48.4917 61.6139 48.9346C61.6975 47.9892 61.6714 47.0372 61.536 46.0978V46.1011ZM45.1684 36.8025C44.7984 36.8481 44.4706 36.8937 44.1428 36.9296C44.0324 36.9296 43.9772 36.9914 43.9708 37.0957C43.9643 37.1999 43.9708 37.2064 43.9708 37.2715L45.1684 37.5321V36.8025ZM52.6106 61.9037C51.5315 62.3482 50.4219 62.714 49.2903 62.998C50.4452 62.7979 51.5642 62.427 52.6106 61.8972V61.9037ZM41.1276 40.2581V39.3723C40.9297 39.6203 40.7869 39.9079 40.7089 40.2158L41.1276 40.2581ZM44.7887 38.431L43.8377 38.2193V38.2616L44.5387 38.7893L44.7887 38.431ZM42.5622 40.0595C42.4031 39.7891 42.2928 39.5286 42.1045 39.2583V40.1735L42.5622 40.0595ZM43.4352 39.6784L43.7598 39.4635C43.5451 39.2348 43.2883 39.0501 43.0036 38.9195L43.4352 39.6784ZM39.4918 39.7729L39.732 39.9096C39.8716 39.6458 39.9884 39.4113 40.1182 39.1312L39.4918 39.7729ZM38.9758 37.0175C39.1713 37.0386 39.369 37.0141 39.5535 36.9458C39.4785 36.9027 39.3976 36.8709 39.3133 36.8514C39.1218 36.8025 39.1218 36.8025 38.9855 37.0045L38.9758 37.0175ZM58.0664 37.7275C58.0859 38.0532 58.1054 38.076 58.4105 38.1639L58.0664 37.7275ZM42.2116 35.148V35.2066C42.3804 35.3141 42.4291 35.3044 42.5135 35.1285L42.2116 35.148ZM44.2791 35.9557L44.6556 35.9004C44.5193 35.7636 44.5193 35.7636 44.2888 35.9427L44.2791 35.9557ZM39.0309 38.8349C38.8167 38.8967 38.8167 38.8967 38.807 39.0433L39.0309 38.8349ZM41.001 35.3955V35.4477L41.212 35.3793V35.3369L41.001 35.3955Z"
  //     fill="white"
  //   />
  //   <path
  //     d="M45.0847 101.857C44.8868 101.815 44.6888 101.779 44.4908 101.733C43.6977 101.578 42.9602 101.213 42.3534 100.678C41.7467 100.142 41.2925 99.4543 41.0375 98.6848C40.3526 96.7763 40.5701 94.9491 41.8391 93.3304C42.2481 92.7985 42.7726 92.3672 43.3727 92.0692C43.9728 91.7712 44.6327 91.6144 45.3022 91.6108C46.3111 91.5447 47.307 91.8673 48.0869 92.513C48.1973 92.6074 48.3011 92.7051 48.4407 92.8386V91.9658C49.5735 91.9032 50.7087 91.8978 51.8421 91.9495V101.512H48.4602V100.518C48.4201 100.54 48.3821 100.565 48.3466 100.593C47.7966 101.238 47.0319 101.661 46.1947 101.782C46.1006 101.798 46.0065 101.828 45.9124 101.854L45.0847 101.857ZM44.1954 96.6916C44.1871 96.9818 44.2373 97.2708 44.343 97.5411C44.4487 97.8114 44.6076 98.0575 44.8104 98.2646C45.0132 98.4717 45.2556 98.6356 45.523 98.7464C45.7904 98.8573 46.0774 98.9128 46.3668 98.9096C46.9435 98.9122 47.4976 98.6847 47.9072 98.2774C48.3169 97.87 48.5485 97.3159 48.551 96.7372C48.5536 96.1585 48.327 95.6024 47.921 95.1913C47.515 94.7803 46.963 94.5479 46.3862 94.5453C46.101 94.5383 45.8173 94.5887 45.5518 94.6935C45.2862 94.7982 45.0443 94.9552 44.8401 95.1552C44.636 95.3552 44.4738 95.5942 44.3631 95.858C44.2524 96.1219 44.1954 96.4053 44.1954 96.6916ZM18.5227 96.532V101.841H14.9006C14.8519 101.655 14.8357 88.1259 14.8811 87.7383C14.9413 87.7284 15.002 87.7218 15.0629 87.7188H20.72C21.3864 87.7097 22.0482 87.8326 22.6673 88.0803C23.3132 88.3277 23.8818 88.7432 24.3147 89.2841C24.7477 89.825 25.0293 90.4718 25.1307 91.1581C25.2596 91.8415 25.2486 92.544 25.0983 93.223C24.9847 93.8178 24.7166 94.3721 24.3211 94.8296C23.9256 95.2871 23.4167 95.6317 22.8458 95.8285C22.6349 95.9099 22.4142 95.9718 22.1643 96.0532C22.8913 97.0303 23.6248 97.9683 24.3518 98.9193C25.0788 99.8704 25.8123 100.831 26.5653 101.818C26.4647 101.818 26.4063 101.838 26.3446 101.838H22.2259C22.1591 101.841 22.0927 101.824 22.0351 101.79C21.9774 101.756 21.931 101.706 21.9014 101.645C20.852 100.008 19.8004 98.3711 18.7466 96.7339C18.6947 96.6525 18.6395 96.5744 18.5844 96.4962L18.5227 96.532ZM21.5671 92.3501C21.6171 91.9966 21.5838 91.6363 21.4697 91.2981C21.398 91.0513 21.2738 90.8231 21.1057 90.6291C20.9376 90.4351 20.7296 90.28 20.496 90.1745C20.1768 90.0319 19.8324 89.9544 19.4831 89.9466C19.1337 89.9387 18.7863 90.0007 18.461 90.1289C18.1928 90.2267 17.951 90.386 17.755 90.594C17.5589 90.8021 17.4139 91.0532 17.3315 91.3274C17.2517 91.6178 17.2167 91.9188 17.2277 92.2198C17.222 92.6373 17.3497 93.0457 17.5921 93.385C17.8345 93.7244 18.1788 93.9769 18.5746 94.1056C18.8749 94.2244 19.1967 94.2788 19.5192 94.2653C19.8417 94.2518 20.1579 94.1708 20.4473 94.0274C20.9444 93.7555 21.3193 93.3035 21.4957 92.7637C21.5161 92.7031 21.5324 92.641 21.5443 92.5781H20.1779C19.8307 93.2067 19.5775 93.3467 19.1199 93.1741C18.961 93.1125 18.8301 92.9947 18.7518 92.8429C18.6734 92.6912 18.6531 92.5159 18.6947 92.3501H21.5671ZM69.9104 93.4021C69.9721 93.3402 70.0337 93.2783 70.0889 93.2132C70.3968 92.8579 70.7759 92.5719 71.2015 92.3737C71.6272 92.1756 72.0897 92.0699 72.5588 92.0635C73.0864 92.0275 73.6164 92.067 74.1329 92.1808C74.6968 92.3041 75.2133 92.5878 75.6208 92.9979C76.0282 93.4079 76.3095 93.927 76.4308 94.4932C76.5274 94.8477 76.5776 95.2134 76.5801 95.581C76.5963 97.6166 76.5801 99.6554 76.5801 101.691V101.851H73.2112V101.564V97.0629C73.2117 96.7052 73.1846 96.3481 73.13 95.9946C73.0954 95.6401 72.9253 95.3129 72.6554 95.0815C72.3855 94.8501 72.0368 94.7327 71.6825 94.7537C71.5446 94.751 71.4068 94.7598 71.2703 94.7798C70.9199 94.8247 70.5951 94.9877 70.3492 95.2422C70.1033 95.4967 69.9508 95.8274 69.9169 96.1803C69.8872 96.3896 69.872 96.6007 69.8714 96.8121V101.857H66.535C66.4863 101.691 66.4636 86.7645 66.5187 86.3118H69.8617V93.3858L69.9104 93.4021ZM39.8593 98.7304V101.798C39.6776 101.854 32.3684 101.877 31.8946 101.828C31.8459 101.652 31.8232 88.2203 31.8719 87.7546C32.0439 87.6992 39.5932 87.6894 39.8561 87.7546V90.8291H35.5329V93.1937H39.6126V96.3008H35.5362V98.7402L39.8593 98.7304ZM60.0632 92.1873V89.4221C60.2482 89.3668 63.1368 89.3537 63.4549 89.4059V92.2524H65.3568V95.1022C64.7401 95.1185 64.117 95.1022 63.4614 95.1022V101.86H60.0697V100.743V95.0566C59.9038 94.9953 59.7319 94.9517 59.5569 94.9263C59.0958 94.8872 58.6322 94.8872 58.171 94.9263C57.1973 95.0501 56.6164 95.6461 56.4152 96.633C56.3492 96.9546 56.3144 97.2818 56.3113 97.6101V101.727H52.9359V92.1938C53.1144 92.1482 55.9283 92.1319 56.2951 92.1938V93.6366L56.3373 93.6529C56.3795 93.5975 56.4249 93.5487 56.4671 93.4933C56.7458 93.1002 57.0946 92.7623 57.4959 92.4967C57.8254 92.2828 58.2045 92.1583 58.5962 92.1352C58.8394 92.1221 59.0832 92.1221 59.3265 92.1352C59.5666 92.1221 59.8036 92.171 60.0632 92.1873ZM28.4899 97.0206C28.9229 97.0199 29.3463 97.1482 29.7065 97.3891C30.0667 97.6301 30.3476 97.9729 30.5136 98.3742C30.6796 98.7754 30.7232 99.2171 30.6389 99.6432C30.5546 100.069 30.3461 100.461 30.04 100.768C29.7339 101.075 29.3438 101.284 28.9192 101.369C28.4945 101.453 28.0544 101.41 27.6546 101.243C27.2547 101.077 26.9131 100.795 26.673 100.433C26.4328 100.072 26.305 99.6469 26.3057 99.2125C26.3048 98.9244 26.3607 98.639 26.4702 98.3726C26.5796 98.1063 26.7405 97.8643 26.9435 97.6606C27.1465 97.4569 27.3876 97.2955 27.653 97.1856C27.9184 97.0758 28.2029 97.0197 28.4899 97.0206ZM28.4997 90.7054C28.9327 90.7067 29.3557 90.8369 29.7149 91.0796C30.0741 91.3224 30.3535 91.6666 30.5176 92.0688C30.6817 92.471 30.7231 92.913 30.6366 93.3388C30.5502 93.7646 30.3397 94.1551 30.0319 94.4608C29.7241 94.7665 29.3328 94.9736 28.9075 95.0559C28.4823 95.1383 28.0423 95.0921 27.6433 94.9233C27.2442 94.7544 26.9041 94.4705 26.666 94.1075C26.4278 93.7446 26.3024 93.3188 26.3057 92.8843C26.3061 92.5963 26.3633 92.3113 26.474 92.0456C26.5846 91.78 26.7466 91.5389 26.9506 91.3364C27.1545 91.1339 27.3964 90.9738 27.6623 90.8655C27.9282 90.7572 28.2128 90.7028 28.4997 90.7054Z"
  //     fill="white"
  //   />
  //   <path
  //     d="M19.9086 91.3666C19.9071 91.4427 19.8906 91.5178 19.8602 91.5876C19.8298 91.6574 19.786 91.7204 19.7313 91.7732C19.6766 91.826 19.6121 91.8674 19.5414 91.8952C19.4708 91.9229 19.3953 91.9364 19.3195 91.9349C19.2436 91.9334 19.1688 91.917 19.0993 91.8864C19.0297 91.8559 18.9669 91.812 18.9143 91.7571C18.8617 91.7022 18.8204 91.6374 18.7927 91.5665C18.7651 91.4956 18.7516 91.4199 18.7531 91.3438C18.7606 91.1929 18.8259 91.0507 18.9354 90.947C19.0449 90.8432 19.19 90.7859 19.3406 90.7869C19.491 90.7927 19.6336 90.8559 19.7392 90.9636C19.8448 91.0714 19.9054 91.2155 19.9086 91.3666Z"
  //     fill="white"
  //   />
  // </svg>
  <svg
    className={className}
    id={id}
    width="24"
    height="24"
    viewBox="0 0 24 24"
    fill="none"
    style={style}
    xmlns="http://www.w3.org/2000/svg">
    <g>
      <rect width="24" height="24" fill="none" />
      <mask
        id="path-1-outside-1_43_3"
        maskUnits="userSpaceOnUse"
        x="4"
        y="-1"
        width="18"
        height="25"
        fill="black">
        <rect fill="white" x="4" y="-1" width="18" height="25" />
        <path d="M16.8591 11.288H7.35513L17.1471 1.56H6.33113V23H5.37113V0.599998H19.4831L9.69113 10.328H16.8591V11.288Z" />
      </mask>
      <path
        d="M16.8591 11.288H7.35513L17.1471 1.56H6.33113V23H5.37113V0.599998H19.4831L9.69113 10.328H16.8591V11.288Z"
        fill="currentColor"
      />
      <path
        d="M16.8591 11.288V12.288H17.8591V11.288H16.8591ZM7.35513 11.288L6.65034 10.5786L4.92967 12.288H7.35513V11.288ZM17.1471 1.56L17.8519 2.26942L19.5726 0.559999H17.1471V1.56ZM6.33113 1.56V0.559999H5.33113V1.56H6.33113ZM6.33113 23V24H7.33113V23H6.33113ZM5.37113 23H4.37113V24H5.37113V23ZM5.37113 0.599998V-0.400002H4.37113V0.599998H5.37113ZM19.4831 0.599998L20.1879 1.30942L21.9086 -0.400002H19.4831V0.599998ZM9.69113 10.328L8.98634 9.61858L7.26567 11.328H9.69113V10.328ZM16.8591 10.328H17.8591V9.328H16.8591V10.328ZM16.8591 10.288H7.35513V12.288H16.8591V10.288ZM8.05991 11.9974L17.8519 2.26942L16.4423 0.850578L6.65034 10.5786L8.05991 11.9974ZM17.1471 0.559999H6.33113V2.56H17.1471V0.559999ZM5.33113 1.56V23H7.33113V1.56H5.33113ZM6.33113 22H5.37113V24H6.33113V22ZM6.37113 23V0.599998H4.37113V23H6.37113ZM5.37113 1.6H19.4831V-0.400002H5.37113V1.6ZM18.7783 -0.109423L8.98634 9.61858L10.3959 11.0374L20.1879 1.30942L18.7783 -0.109423ZM9.69113 11.328H16.8591V9.328H9.69113V11.328ZM15.8591 10.328V11.288H17.8591V10.328H15.8591Z"
        fill="currentColor"
        mask="url(#path-1-outside-1_43_3)"
      />
    </g>
  </svg>
);

export { FlowLogo };
